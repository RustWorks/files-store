use actix_files::NamedFile;
use actix_web::{
    get,
    web::{Data, Path},
};
use sqlx::PgPool;
use users::domain::User;
use uuid::Uuid;

use crate::domain::FsNodeType;
use crate::errors::ApiError;
use crate::repositories::FsNodeStore;
use crate::storages::{LocalStorage, Storage};

#[get("/api/files/download/{file_uuid}")]
async fn download(
    pool: Data<PgPool>,
    local_storage: Data<LocalStorage>,
    file_uuid: Path<Uuid>,
    user: User,
) -> Result<NamedFile, ApiError> {
    let mut connection = pool.acquire().await?;
    let fs_node = connection
        .find_fs_node_by_uuid(&file_uuid, &FsNodeType::File, &user.uuid)
        .await?;
    let file = local_storage.get_file(&fs_node.uuid, &user.uuid).await?;
    let named_file = NamedFile::from_file(file.into_std().await, &fs_node.name)?;
    Ok(named_file)
}
