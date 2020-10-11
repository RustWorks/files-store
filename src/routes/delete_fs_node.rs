use actix_web::{
    delete,
    web::{Data, HttpResponse, Path},
};
use log::debug;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::FsNodeStore;

#[delete("/api/files/{parent_uuid}")]
async fn delete_fs_node_route(
    pool: Data<PgPool>,
    parent_uuid: Path<Uuid>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.begin().await?;

    let fs_node = connection
        .find_any_fs_node_by_uuid(&parent_uuid, &user)
        .await?;

    debug!("Fs node to delete = {:?}", fs_node);

    if fs_node.parent_id.is_some() {
        let updated = connection
            .update_deleteed_fs_node(fs_node.id, &user)
            .await?;

        let deleted = connection.delete_fs_node(fs_node.id).await?;

        let json = json!({
            "updated": updated,
            "deleted": deleted,
        });

        connection.commit().await?;

        Ok(HttpResponse::Ok().json(json))
    } else {
        connection.rollback().await?;

        Err(ApiError::Invalid {
            message: "You can't delete root directory".to_string(),
        })
    }
}
