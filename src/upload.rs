use actix_multipart::{Field, Multipart};
use actix_web::{
    post,
    web::{Data, HttpResponse},
};
use async_std::prelude::*;
use blake2::{Blake2s, Digest};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;

use crate::config::Config;
use crate::errors::ApiError;
use crate::files_repository::FilesStore;
use crate::uploaded_file::{UploadFile, UploadedFile};

fn get_upload_file(field: &Field) -> Option<UploadFile> {
    let content_disposition = field.content_disposition()?;
    let filename = content_disposition.get_filename()?;
    let filename = sanitize_filename::sanitize(filename);
    let content_type = field.content_type().to_string();
    let upload_file = UploadFile::new(filename, content_type);
    Some(upload_file)
}

#[post("/api/upload")]
async fn upload(
    mut multipart: Multipart,
    config: Data<Config>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let mut uploaded_files = Vec::new();
    while let Ok(Some(mut field)) = multipart.try_next().await {
        let upload_file = get_upload_file(&field).ok_or(ApiError::Invalid {
            message: "should content a filename".to_string(),
        })?;
        let path = format!("{}/{}", config.local_storage_path, upload_file.filename);
        let mut file = async_std::fs::File::create(format!(
            "{}/{}",
            config.local_storage_path, upload_file.filename
        ))
        .await?;
        let mut size: usize = 0;
        let mut hasher = Blake2s::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| ApiError::InternalServer)?;
            size += data.len();
            hasher.update(&data);
            file.write_all(&data).await?;
        }
        let uploaded_file = UploadedFile::new(
            path,
            upload_file.filename,
            format!("{:02x}", hasher.finalize()),
            size as i64,
            upload_file.content_type,
        );
        let uploaded_file = pool.acquire().await?.insert(&uploaded_file).await?;
        uploaded_files.push(uploaded_file);
    }
    Ok(HttpResponse::Ok().json(uploaded_files))
}
