use async_std::fs::{DirBuilder, File};
use async_std::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::storages::{Storage, Uploader};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    pub local_storage_path: String,
}

impl LocalStorage {
    pub async fn new(local_storage_path: &str) -> Result<Self, ApiError> {
        let path = async_std::path::Path::new(local_storage_path);
        if path.exists().await {
            let local_storage = Self {
                local_storage_path: local_storage_path.to_owned(),
            };
            Ok(local_storage)
        } else {
            Err(ApiError::InternalServer)
        }
    }

    async fn create_file(&self, uuid: &Uuid, user_uuid: &Uuid) -> Result<File, ApiError> {
        let directory_path = format!("{}/{}", self.local_storage_path, user_uuid);
        DirBuilder::new()
            .recursive(true)
            .create(&directory_path)
            .await?;
        let path = format!("{}/{}", directory_path, uuid);
        let path = async_std::path::Path::new(&path);
        let file = async_std::fs::File::create(path).await?;
        Ok(file)
    }
}

pub struct LocalUploader {
    file: File,
}

#[async_trait]
impl Uploader for LocalUploader {
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), ApiError> {
        self.file.write_all(buf).await?;
        Ok(())
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn get_uploader(
        &self,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<Box<dyn Uploader>, ApiError> {
        let file = self.create_file(uuid, user_uuid).await?;
        let uploader = LocalUploader { file };
        Ok(Box::new(uploader))
    }

    async fn get_file(&self, uuid: &Uuid, user_uuid: &Uuid) -> Result<tokio::fs::File, ApiError> {
        let path = format!("{}/{}/{}", self.local_storage_path, user_uuid, uuid);
        let file = tokio::fs::File::open(&path).await?;
        Ok(file)
    }

    async fn remove_file(&self, uuid: &Uuid, user_uuid: &Uuid) -> Result<(), ApiError> {
        let path = format!("{}/{}/{}", self.local_storage_path, user_uuid, uuid);
        Ok(async_std::fs::remove_file(path).await?)
    }

    async fn create_thumbnail_file(
        &self,
        thumbnail_uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<tokio::fs::File, ApiError> {
        let path = format!(
            "{}/{}/{}",
            self.local_storage_path, user_uuid, thumbnail_uuid
        );
        let file = tokio::fs::File::create(&path).await?;
        Ok(file)
    }
}
