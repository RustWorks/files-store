use actix_web::{
    post,
    web::{Data, HttpResponse, Json},
};
use files_store_domain::{CreateFsNode, CreateFsNodeDirectory, FsNode, FsNodeMetadata, FsNodeType};
use sqlx::PgPool;
use users::domain::User;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::repositories::FsNodeStore;

#[post("/api/fs/directories")]
async fn create_directory(
    pool: Data<PgPool>,
    payload: Json<CreateFsNodeDirectory>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut tx = pool.begin().await?;
    let parent_directory = match payload.parent_uuid {
        Some(ref parent_uuid) => {
            tx.find_fs_node_by_uuid(parent_uuid, &FsNodeType::Directory, &user.uuid)
                .await?
        }
        None => tx.find_root_fs_node(&FsNodeType::Root, &user.uuid).await?,
    };
    let uuid = Uuid::new_v4();
    let parent_id = parent_directory.id;
    let name = payload.name.clone();
    let create_stored_fs_node = CreateFsNode::new(
        uuid,
        parent_id,
        FsNodeType::Directory,
        name,
        FsNodeMetadata::Directory,
    );
    let directory: FsNode = tx
        .insert_fs_node(create_stored_fs_node, &user.uuid)
        .await?
        .into();
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(directory))
}
