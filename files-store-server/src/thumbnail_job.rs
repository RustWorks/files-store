use actix::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::repositories::{
    CreateStoredFsNode, FsNodeMetadata, FsNodeStore, FsNodeType, StoredFsNode,
};
use crate::storages::{LocalStorage, Storage};

#[derive(Debug)]
pub struct ThumbnailActor {
    pool: PgPool,
    local_storage: LocalStorage,
}

pub type ThumbnailActorAddr = Addr<ThumbnailActor>;

impl ThumbnailActor {
    pub fn new(pool: PgPool, local_storage: LocalStorage) -> Self {
        Self {
            pool,
            local_storage,
        }
    }

    pub fn start(pool: PgPool, local_storage: LocalStorage) -> ThumbnailActorAddr {
        ThumbnailActor::new(pool, local_storage).start()
    }
}

impl Actor for ThumbnailActor {
    type Context = Context<Self>;
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct CreateThumbnail {
    fs_node_uuid: Uuid,
    fs_node_user_uuid: Uuid,
}
impl CreateThumbnail {
    pub fn new(fs_node_uuid: Uuid, fs_node_user_uuid: Uuid) -> Self {
        Self {
            fs_node_uuid,
            fs_node_user_uuid,
        }
    }
}

async fn handle(
    pool: PgPool,
    local_storage: LocalStorage,
    msg: CreateThumbnail,
) -> Result<StoredFsNode, ApiError> {
    let mut connection = pool.begin().await?;
    let fs_node = connection
        .find_fs_node_by_uuid(&msg.fs_node_uuid, FsNodeType::File, &msg.fs_node_user_uuid)
        .await?;
    let file = local_storage
        .get_file(&fs_node.uuid, &fs_node.user_uuid)
        .await?;

    let buf_reader = std::io::BufReader::new(file.into_std().await);
    let reader = image::io::Reader::new(buf_reader).with_guessed_format()?;

    let format = reader.format();

    let image = reader.decode()?;

    let image = image.thumbnail(200, 200);

    let content_type = match fs_node.metadata.0 {
        FsNodeMetadata::File { content_type, .. } => content_type,
        _ => "".to_string(),
    };

    let metadata = FsNodeMetadata::new_thumbnail(content_type, 0, 200, 200);

    let create_stored_fs_node = CreateStoredFsNode::new(
        uuid::Uuid::new_v4(),
        fs_node.id,
        FsNodeType::Thumbnail,
        fs_node.name,
        metadata,
    );

    let mut file = local_storage
        .create_thumbnail_file(&create_stored_fs_node.uuid, &fs_node.user_uuid)
        .await?
        .into_std()
        .await;

    let _ = actix_web::web::block(move || image.write_to(&mut file, format.unwrap())).await?;

    let stored_fs_node = connection
        .insert(create_stored_fs_node, &fs_node.user_uuid)
        .await?;

    let _ = connection.commit().await?;
    Ok(stored_fs_node)
}

async fn result_wrapper(pool: PgPool, local_storage: LocalStorage, msg: CreateThumbnail) {
    match handle(pool, local_storage, msg).await {
        Ok(stored_fs_node) => tracing::debug!("Thumbnail created: {:#?}", stored_fs_node),
        Err(error) => tracing::error!("Thumbnail creation error: {:#?}", &error),
    }
}

impl Handler<CreateThumbnail> for ThumbnailActor {
    type Result = ();

    fn handle(&mut self, msg: CreateThumbnail, ctx: &mut Context<Self>) -> Self::Result {
        let _ = ctx.spawn(actix::fut::wrap_future(result_wrapper(
            self.pool.clone(),
            self.local_storage.clone(),
            msg,
        )));
    }
}
