use actix_multipart::{Field, Multipart};
use actix_web::{
    post,
    web::{Data, HttpResponse, Path},
};
use blake2::{Blake2s, Digest};
use files_store_domain::{CreateFsNode, FsNode, FsNodeMetadata, FsNodeType};
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;
use tracing::debug;
use users::domain::User;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::jobs::thumbnail_job::{CreateThumbnail, ThumbnailActorAddr};
use crate::repositories::FsNodeStore;
use crate::storages::{LocalStorage, Storage};

fn get_filename(field: &Field) -> Option<String> {
    let content_disposition = field.content_disposition()?;
    let filename = content_disposition.get_filename()?;
    let filename = sanitize_filename::sanitize(filename);
    Some(filename)
}

#[derive(Debug, Serialize)]
struct UploadError {
    pub filename: String,
    pub error_message: String,
}

impl UploadError {
    pub fn new(filename: String, error: ApiError) -> Self {
        Self {
            filename,
            error_message: error.to_string(),
        }
    }
}

#[post("/api/fs/upload/{parent_uuid}")]
async fn upload(
    mut multipart: Multipart,
    pool: Data<PgPool>,
    local_storage: Data<LocalStorage>,
    parent_uuid: Path<Uuid>,
    user: User,
    thumbnail_job: Data<ThumbnailActorAddr>,
) -> Result<HttpResponse, ApiError> {
    let mut tx = pool.begin().await?;
    let mut uploaded_files: Vec<Value> = vec![];
    while let Ok(Some(mut field)) = multipart.try_next().await {
        let filename = get_filename(&field).ok_or(ApiError::Invalid {
            message: "should content a filename".to_string(),
        })?;
        let file_uuid = Uuid::new_v4();
        let parent_directory = tx
            .find_fs_node_by_uuid(&parent_uuid, &FsNodeType::Directory, &user.uuid)
            .await?;
        let maybe_existing_fs_node = tx
            .find_fs_node_by_name(parent_directory.id, &filename, &user.uuid)
            .await?;
        if maybe_existing_fs_node.is_none() {
            let ancestors = tx
                .find_fs_nodes_ancestor_by_id(parent_directory.id, &user.uuid)
                .await?;
            let path = itertools::join(ancestors.into_iter().map(|a| a.name), "/");

            let mut uploder = local_storage.get_uploader(&file_uuid, &user.uuid).await?;
            let mut size: usize = 0;
            let mut hasher = Blake2s::new();
            debug!("uploade file filename={} path={}", &filename, &path);
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|_| ApiError::InternalServer)?;
                size += data.len();
                hasher.update(&data);
                uploder.write_all(&data).await?;
            }
            let hash = format!("{:02x}", hasher.finalize());
            let content_type = field.content_type().to_string();
            let file_fs_node_metadata =
                FsNodeMetadata::new_file(&filename, hash, content_type.clone(), size as i64);
            let create_stored_fs_node = CreateFsNode::new(
                file_uuid,
                parent_directory.id,
                FsNodeType::File,
                filename,
                file_fs_node_metadata,
            );
            let stored_fs_node = tx.insert_fs_node(create_stored_fs_node, &user.uuid).await?;
            let fs_node: FsNode = stored_fs_node.into();
            if (&content_type == "image/jpeg") || (&content_type == "image/png") {
                let _ = thumbnail_job
                    .send(CreateThumbnail::new(fs_node.uuid, fs_node.user_uuid))
                    .await?;
            }
            uploaded_files.push(serde_json::to_value(&fs_node)?);
        } else {
            uploaded_files.push(serde_json::to_value(UploadError::new(
                filename,
                ApiError::Duplicate,
            ))?);
        }
    }
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(uploaded_files))
}
