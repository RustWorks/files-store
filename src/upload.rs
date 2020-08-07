use actix_multipart::{Field, Multipart};
use actix_web::{
    post,
    web::{Data, HttpResponse, Path},
};
use blake2::{Blake2s, Digest};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;

use crate::errors::ApiError;
use crate::files_repository::FilesStore;
use crate::sanitize_path::sanitize_path;
use crate::storage::{LocalStorage, Storage};
use crate::uploaded_file::{UploadFile, UploadedFile};

fn get_upload_file(directory: &str, field: &Field) -> Option<UploadFile> {
    let content_disposition = field.content_disposition()?;
    let filename = content_disposition.get_filename()?;
    let filename = sanitize_filename::sanitize(filename);
    let content_type = field.content_type().to_string();
    let directory = sanitize_path(directory);
    let directory = format!("/{}", directory);
    let path = if directory == "/" {
        format!("{}{}", directory, filename)
    } else {
        format!("{}/{}", directory, filename)
    };
    let upload_file = UploadFile::new(path, directory, filename, content_type);
    Some(upload_file)
}

#[post("/api/upload/{tail:.*}")]
async fn upload(
    mut multipart: Multipart,
    pool: Data<PgPool>,
    local_storage: Data<LocalStorage>,
    directory: Path<String>,
) -> Result<HttpResponse, ApiError> {
    let mut tx = pool.begin().await?;
    let uploaded_file = if let Ok(Some(mut field)) = multipart.try_next().await {
        let upload_file = get_upload_file(&directory, &field).ok_or(ApiError::Invalid {
            message: "should content a filename".to_string(),
        })?;
        dbg!(&upload_file);
        if !tx.exists(&upload_file.path).await? {
            let mut uploder = local_storage.get_uploader(&upload_file).await?;
            let mut size: usize = 0;
            let mut hasher = Blake2s::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|_| ApiError::InternalServer)?;
                size += data.len();
                hasher.update(&data);
                uploder.write_all(&data).await?;
            }
            let uploaded_file = UploadedFile::from(
                upload_file,
                "local_storage".to_string(),
                format!("{:02x}", hasher.finalize()),
                size as i64,
            );
            let uploaded_file = tx.insert(&uploaded_file).await?;
            Ok(uploaded_file)
        } else {
            Err(ApiError::Duplicate)
        }
    } else {
        Err(ApiError::Invalid {
            message: "Should have a file".to_string(),
        })
    }?;
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(uploaded_file))
}