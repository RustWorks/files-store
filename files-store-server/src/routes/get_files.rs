use actix_web::{
    get,
    web::{Data, HttpResponse, Path},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;
use crate::errors::ApiError;
use crate::repositories::{FsNodeStore, FsNodeType, FsNodesRespose};

#[get("/api/files/{parent_uuid}")]
async fn get_files(
    pool: Data<PgPool>,
    parent_uuid: Path<Uuid>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let parent_directory = connection
        .find_fs_node_by_uuid(&parent_uuid, FsNodeType::Directory, &user.uuid)
        .await?;
    let ancestors = connection
        .find_fs_nodes_ancestor_by_id(parent_directory.id, &user)
        .await?;
    let fs_nodes = connection
        .find_fs_nodes_by_parent_id(parent_directory.id, &user)
        .await?;
    let response = FsNodesRespose::new(parent_directory, fs_nodes, ancestors);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/api/files")]
async fn get_root_files(pool: Data<PgPool>, user: User) -> Result<HttpResponse, ApiError> {
    let mut connection = pool.acquire().await?;
    let parent_directory = connection
        .find_root_fs_node(FsNodeType::Root, &user)
        .await?;
    let fs_nodes = connection
        .find_fs_nodes_by_parent_id(parent_directory.id, &user)
        .await?;
    let ancestors = vec![parent_directory.clone()];
    let response = FsNodesRespose::new(parent_directory, fs_nodes, ancestors);
    Ok(HttpResponse::Ok().json(response))
}
