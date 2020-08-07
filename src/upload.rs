use actix_multipart::{Field, Multipart};
use actix_web::{
    post,
    web::{Path, Data, HttpResponse},
};
use blake2::{Blake2s, Digest};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;

use crate::storage::{LocalStorage, Storage};
use crate::errors::ApiError;
use crate::files_repository::FilesStore;
use crate::uploaded_file::{UploadFile, UploadedFile};
use crate::sanitize_path::sanitize_path;

fn get_upload_file(path: &str, field: &Field) -> Option<UploadFile> {
    let content_disposition = field.content_disposition()?;
    let filename = content_disposition.get_filename()?;
    let filename = sanitize_filename::sanitize(filename);
    let content_type = field.content_type().to_string();
    let path = sanitize_path(path);
    let upload_file = UploadFile::new(path, filename, content_type);
    Some(upload_file)
}

#[post("/api/upload/{tail:.*}")]
async fn upload(
    mut multipart: Multipart,
    pool: Data<PgPool>,
    local_storage: Data<LocalStorage>,
    path: Path<String>
) -> Result<HttpResponse, ApiError> {
    let mut tx = pool.begin().await?;
    let uploaded_file = if let Ok(Some(mut field)) = multipart.try_next().await {
        let upload_file = get_upload_file(&path, &field).ok_or(ApiError::Invalid {
            message: "should content a filename".to_string(),
        })?;
        if !tx.exists(&format!("/{}/{}", upload_file.path, upload_file.filename)).await? {
            let mut uploder = local_storage.uploader(&upload_file).await?;
            let mut size: usize = 0;
            let mut hasher = Blake2s::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|_| ApiError::InternalServer)?;
                size += data.len();
                hasher.update(&data);
                uploder.write_all(&data).await?;
            }
            let uploaded_file = UploadedFile::new(
                format!("/{}/{}", upload_file.path, upload_file.filename),
                upload_file.filename,
                format!("/{}", upload_file.path),
                "local_storage".to_string(),
                format!("{:02x}", hasher.finalize()),
                size as i64,
                upload_file.content_type,
            );
            let uploaded_file = tx.insert(&uploaded_file).await?;
            Ok(uploaded_file)
        } else {
            Err(ApiError::Duplicate)
        }
    } else {
        Err(ApiError::Invalid { message: "Should have a file".to_string() })
    }?;
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(uploaded_file))
}
