use async_std::fs::{DirBuilder, File};
use async_std::prelude::*;
use async_trait::async_trait;

use crate::errors::ApiError;
use crate::uploaded_file::{UploadFile, UploadedFile};

#[async_trait]
pub trait Storage {
    async fn get_uploader(&self, file: &UploadFile) -> Result<Box<dyn Uploader>, ApiError>;
    async fn get_downloader(&self, file: &UploadedFile) -> Result<tokio::fs::File, ApiError>;
}

#[async_trait]
pub trait Uploader {
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), ApiError>;
}

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

    async fn create_file(&self, upload_file: &UploadFile) -> Result<File, ApiError> {
        let directory_path = format!("{}{}", self.local_storage_path, upload_file.directory);
        DirBuilder::new()
            .recursive(true)
            .create(&directory_path)
            .await?;
        let path = if upload_file.directory == "/" {
            format!("{}{}", directory_path, upload_file.filename)
        } else {
            format!("{}/{}", directory_path, upload_file.filename)
        };
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
    async fn get_uploader(&self, upload_file: &UploadFile) -> Result<Box<dyn Uploader>, ApiError> {
        let file = self.create_file(upload_file).await?;
        let uploader = LocalUploader { file };
        Ok(Box::new(uploader))
    }

    async fn get_downloader(&self, file: &UploadedFile) -> Result<tokio::fs::File, ApiError> {
        let path = format!("{}{}", self.local_storage_path, file.path);
        let file = tokio::fs::File::open(&path).await?;
        Ok(file)
    }
}
