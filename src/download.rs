use actix_files::NamedFile;
use actix_web::{
    get,
    web::{Data, Path},
};
use sqlx::PgPool;

use crate::errors::ApiError;
use crate::files_repository::FilesStore;
use crate::storage::{LocalStorage, Storage};

#[get("/api/download/{tail:.*}")]
async fn download(
    pool: Data<PgPool>,
    local_storage: Data<LocalStorage>,
    path: Path<String>,
) -> Result<NamedFile, ApiError> {
    let mut connection = pool.acquire().await?;
    let path = format!("/{}", path);
    let uploaded_file = connection.find_file_by_path(&path).await?;
    let file = local_storage.get_downloader(&uploaded_file).await?;
    let named_file = NamedFile::from_file(file.into_std().await, &uploaded_file.name)?;
    Ok(named_file)
}
