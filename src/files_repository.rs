use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::FromRow;
use sqlx::{query_as, PgConnection};

use crate::repository_erros::RepositoryError;
use crate::uploaded_file::UploadedFile;

#[derive(Debug, FromRow)]
pub struct Count {
    pub count: i64,
}

impl Count {
    pub fn is_existe(&self) -> bool {
        self.count > 0
    }
}

#[async_trait]
pub trait FilesStore {
    async fn insert(&mut self, file: &UploadedFile) -> Result<UploadedFile, RepositoryError>;
    async fn exists(&mut self, path: &str) -> Result<bool, RepositoryError>;
    async fn find_file_by_path(&mut self, path: &str) -> Result<UploadedFile, RepositoryError>;
}

#[async_trait]
impl FilesStore for PgConnection {
    async fn insert(
        &mut self,
        uploaded_file: &UploadedFile,
    ) -> Result<UploadedFile, RepositoryError> {
        let uploaded_file = query_as(
            r#"
            INSERT INTO files (uuid, owner, path, name, directory, storage, hash, size, content_type, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *
            "#,
        )
        .bind(uploaded_file.uuid)
        .bind(uploaded_file.owner)
        .bind(&uploaded_file.path)
        .bind(&uploaded_file.name)
        .bind(&uploaded_file.directory)
        .bind(&uploaded_file.storage)
        .bind(&uploaded_file.hash)
        .bind(uploaded_file.size)
        .bind(&uploaded_file.content_type)
        .bind(&uploaded_file.metadata)
        .bind(uploaded_file.created_at)
        .bind(uploaded_file.updated_at)
        .fetch_one(self)
        .await?;
        Ok(uploaded_file)
    }

    async fn find_file_by_path(&mut self, path: &str) -> Result<UploadedFile, RepositoryError> {
        let uploaded_file = query_as("SELECT * FROM files WHERE path = $1")
            .bind(path)
            .fetch_one(self)
            .await?;
        Ok(uploaded_file)
    }

    async fn exists(&mut self, path: &str) -> Result<bool, RepositoryError> {
        let existe: Count = query_as(
            r#"
            SELECT COUNT(*) as count
            FROM files
            WHERE path = $1
            "#,
        )
        .bind(path)
        .fetch_one(self)
        .await?;
        Ok(existe.is_existe())
    }
}
