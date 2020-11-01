use actix_web::{
    delete,
    web::{Data, HttpResponse, Path},
};
use sqlx::PgPool;
use tracing::debug;
use users::domain::User;
use uuid::Uuid;

use crate::domain::FsNodeType;
use crate::errors::ApiError;
use crate::repositories::FsNodeStore;

#[delete("/api/files/{uuid}")]
async fn move_fs_node_to_bin_route(
    pool: Data<PgPool>,
    uuid: Path<Uuid>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.begin().await?;

    let source_fs_node = connection
        .find_any_fs_node_by_uuid(&uuid, &user.uuid)
        .await?;

    let bin = connection
        .find_root_fs_node(&FsNodeType::Bin, &user.uuid)
        .await?;

    debug!(
        "Fs node to move = {:#?} into = {:#?}",
        &source_fs_node, &bin
    );

    if source_fs_node.parent_id.is_some() {
        connection
            .move_fs_node_update_parent_id(source_fs_node.id, bin.id)
            .await?;
        connection
            .move_fs_node_disconnect(source_fs_node.id)
            .await?;
        connection
            .move_fs_node_update_ancestors(source_fs_node.id, bin.id)
            .await?;
        connection
            .update_deleted_at_fs_node(source_fs_node.id, &user.uuid)
            .await?;

        connection.commit().await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        connection.rollback().await?;

        Err(ApiError::Invalid {
            message: "Can't delete".to_string(),
        })
    }
}
