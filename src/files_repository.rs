use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::{query_as, PgConnection};

use crate::repository_erros::RepositoryError;
use crate::uploaded_file::UploadedFile;

#[async_trait]
pub trait FilesStore {
    async fn insert(&mut self, file: &UploadedFile) -> Result<UploadedFile, RepositoryError>;
}

#[async_trait]
impl FilesStore for PgConnection {
    async fn insert(
        &mut self,
        uploaded_file: &UploadedFile,
    ) -> Result<UploadedFile, RepositoryError> {
        let uploaded_file = query_as(
            r#"
            INSERT INTO files (uuid, owner, path, name, hash, size, content_type, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
            "#,
        )
        .bind(uploaded_file.uuid)
        .bind(uploaded_file.owner)
        .bind(&uploaded_file.path)
        .bind(&uploaded_file.name)
        .bind(&uploaded_file.hash)
        .bind(uploaded_file.size)
        .bind(&uploaded_file.content_type)
        .bind(uploaded_file.created_at)
        .bind(uploaded_file.updated_at)
        .fetch_one(self)
        .await?;
        Ok(uploaded_file)
    }
}
