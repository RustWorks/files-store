use actix_web::{
    get,
    web::{Data, HttpResponse, Path},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::{FsNode, FsNodeStore, FsNodeType};

#[get("/api/files/{parent_uuid}")]
async fn get_files(
    pool: Data<PgPool>,
    parent_uuid: Path<Uuid>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let parent_directory = connection
        .find_fs_node_by_uuid(&parent_uuid, FsNodeType::Directory, &user)
        .await?;
    let fs_nodes = connection
        .find_fs_nodes_by_parent_id(parent_directory.id, &user)
        .await?;
    let fs_nodes = fs_nodes
        .into_iter()
        .map(FsNode::from)
        .collect::<Vec<FsNode>>();
    Ok(HttpResponse::Ok().json(fs_nodes))
}

#[get("/api/files")]
async fn get_root_files(pool: Data<PgPool>, user: User) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let parent_directory = connection
        .find_root_fs_node(FsNodeType::Directory, &user)
        .await?;
    let fs_nodes = connection
        .find_fs_nodes_by_parent_id(parent_directory.id, &user)
        .await?;
    let fs_nodes = fs_nodes
        .into_iter()
        .map(FsNode::from)
        .collect::<Vec<FsNode>>();
    Ok(HttpResponse::Ok().json(fs_nodes))
}
