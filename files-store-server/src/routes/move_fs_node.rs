use actix_web::{
    put,
    web::{Data, HttpResponse, Json},
};
use serde::Deserialize;
use sqlx::PgPool;
use std::str::FromStr;
use tracing::debug;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::{FsNodeStore, FsNodeType};

fn can_move(source_fs_node_type: &FsNodeType, destination_fs_node_type: &FsNodeType) -> bool {
    match (source_fs_node_type, destination_fs_node_type) {
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

    let source_fs_node = connection
        .find_any_fs_node_by_uuid(&query.source_uuid, &user)
        .await?;

    let destination_fs_node = connection
        .find_any_fs_node_by_uuid(&query.destination_uuid, &user)
        .await?;

    debug!(
        "Fs node to move = {:#?} into = {:#?}",
        &source_fs_node, &destination_fs_node
    );

    let source_fs_node_type = FsNodeType::from_str(&source_fs_node.node_type)?;
    let destination_fs_node_type = FsNodeType::from_str(&destination_fs_node.node_type)?;

    if source_fs_node.parent_id.is_some()
        && can_move(&source_fs_node_type, &destination_fs_node_type)
        && source_fs_node.id != destination_fs_node.id
    {
        connection
            .move_fs_node_update_parent_id(source_fs_node.id, destination_fs_node.id)
            .await?;
        connection
            .move_fs_node_disconnect(source_fs_node.id)
            .await?;
        connection
            .move_fs_node_update_ancestors(source_fs_node.id, destination_fs_node.id)
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
