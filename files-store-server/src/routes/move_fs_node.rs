use actix_web::{
    put,
    web::{Data, HttpResponse, Json},
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::{FsNodeStore, FsNodeType, StoredFsNode};

fn can_move(sourcet_fs_node: &StoredFsNode, destination_fs_node: &StoredFsNode) -> bool {
    match (sourcet_fs_node.node_type(), destination_fs_node.node_type()) {
        (FsNodeType::Directory, FsNodeType::Directory) => true,
        (FsNodeType::Directory, FsNodeType::Root) => true,
        (FsNodeType::File, FsNodeType::Directory) => true,
        (FsNodeType::File, FsNodeType::Root) => true,
        _ => false,
    }
}

#[derive(Debug, Deserialize)]
struct MoveFsNodeQuery {
    source_uuid: Uuid,
    destination_uuid: Uuid,
}

#[put("/api/files")]
async fn move_fs_node_route(
    pool: Data<PgPool>,
    query: Json<MoveFsNodeQuery>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.begin().await?;

    let sourcet_fs_node = connection
        .find_any_fs_node_by_uuid(&query.source_uuid, &user)
        .await?;

    let destination_fs_node = connection
        .find_any_fs_node_by_uuid(&query.destination_uuid, &user)
        .await?;

    debug!(
        "Fs node to move = {:#?} into = {:#?}",
        &sourcet_fs_node, &destination_fs_node
    );

    if sourcet_fs_node.parent_id.is_some()
        && can_move(&sourcet_fs_node, &destination_fs_node)
        && sourcet_fs_node.id != destination_fs_node.id
    {
        connection
            .move_fs_node_update_parent_id(sourcet_fs_node.id, destination_fs_node.id)
            .await?;
        connection
            .move_fs_node_disconnect(sourcet_fs_node.id)
            .await?;
        connection
            .move_fs_node_update_ancestors(sourcet_fs_node.id, destination_fs_node.id)
            .await?;

        connection.commit().await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        connection.rollback().await?;

        Err(ApiError::Invalid {
            message: "Can't move".to_string(),
        })
    }
}
