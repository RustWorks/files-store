use actix_web::{
    post,
    web::{Data, HttpResponse, Json},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::{CreateStoredFsNode, FsNode, FsNodeStore, FsNodeType};

#[derive(Debug, Deserialize)]
struct CreateDirectoryQuery {
    name: String,
    parent_uuid: Option<Uuid>,
}

#[post("/api/directories")]
async fn create_directory(
    pool: Data<PgPool>,
    payload: Json<CreateDirectoryQuery>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut tx = pool.begin().await?;
    let parent_directory = match payload.parent_uuid {
        Some(ref parent_uuid) => {
            tx.find_fs_node_by_uuid(parent_uuid, FsNodeType::Directory, &user)
                .await?
        }
        None => tx.find_root_fs_node(&user, FsNodeType::Directory).await?,
    };
    dbg!(&parent_directory);
    let parent_id = parent_directory.id;
    let name = payload.name.clone();
    let create_stored_fs_node =
        CreateStoredFsNode::new(parent_id, FsNodeType::Directory, name, json!({}));
    dbg!(&create_stored_fs_node);
    let directory: FsNode = tx.insert(&create_stored_fs_node, &user).await?.into();
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(directory))
}
