use actix::prelude::*;
use chrono::prelude::*;
use files_store_domain::FsNodeType;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::repositories::FsNodeStore;
use crate::storages::{LocalStorage, Storage};

#[derive(Debug)]
pub struct BinCleanerActor {
    pool: PgPool,
    local_storage: LocalStorage,
}

pub type BinCleanerActorAddr = Addr<BinCleanerActor>;

impl BinCleanerActor {
    pub fn new(pool: PgPool, local_storage: LocalStorage) -> Self {
        Self {
            pool,
            local_storage,
        }
    }

    pub fn start(pool: PgPool, local_storage: LocalStorage) -> BinCleanerActorAddr {
        BinCleanerActor::new(pool, local_storage).start()
    }
}

impl Actor for BinCleanerActor {
    type Context = Context<Self>;
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Cleanup {
    pub user_uuid: Uuid,
}

impl Cleanup {
    pub fn new(user_uuid: Uuid) -> Self {
        Self { user_uuid }
    }
}

async fn delete_fs_nodes(
    pool: &PgPool,
    local_storage: &LocalStorage,
    date: &NaiveDateTime,
    fs_node_type: &FsNodeType,
    message: &Cleanup,
) -> Result<(), ApiError> {
    let mut connection = pool.begin().await?;

    let deleted_fs_nodes = connection
        .find_deleted_fs_nodes(date, fs_node_type, &message.user_uuid)
        .await?;

    for fs_node in deleted_fs_nodes {
        if fs_node_type != &FsNodeType::Directory {
            local_storage
                .remove_file(&fs_node.uuid, &fs_node.user_uuid)
                .await?;
        }
        connection.delete_fs_node(fs_node.id).await?;
    }

    connection.commit().await?;
    Ok(())
}

async fn handle(
    pool: PgPool,
    local_storage: LocalStorage,
    message: Cleanup,
) -> Result<(), ApiError> {
    let duration = chrono::Duration::days(30);
    let date = Utc::now() + duration;
    delete_fs_nodes(
        &pool,
        &local_storage,
        &date.naive_utc(),
        &FsNodeType::Thumbnail,
        &message,
    )
    .await?;
    delete_fs_nodes(
        &pool,
        &local_storage,
        &date.naive_utc(),
        &FsNodeType::File,
        &message,
    )
    .await?;
    delete_fs_nodes(
        &pool,
        &local_storage,
        &date.naive_utc(),
        &FsNodeType::Directory,
        &message,
    )
    .await?;
    Ok(())
}

async fn result_wrapper(pool: PgPool, local_storage: LocalStorage, msg: Cleanup) {
    match handle(pool, local_storage, msg).await {
        Ok(result) => tracing::debug!("Bin cleanup succes: {:#?}", result),
        Err(error) => tracing::error!("Bin cleanup error: {:#?}", &error),
    }
}

impl Handler<Cleanup> for BinCleanerActor {
    type Result = ();

    fn handle(&mut self, msg: Cleanup, ctx: &mut Context<Self>) -> Self::Result {
        tracing::debug!("Received Cleanup");
        let _ = ctx.spawn(actix::fut::wrap_future(result_wrapper(
            self.pool.clone(),
            self.local_storage.clone(),
            msg,
        )));
    }
}
