use async_trait::async_trait;

use crate::errors::ApiError;

#[async_trait]
pub trait Storage {
    async fn get_uploader(
        &self,
        directory: &str,
        filename: &str,
    ) -> Result<Box<dyn Uploader>, ApiError>;
    async fn get_downloader(&self, path: &str) -> Result<tokio::fs::File, ApiError>;
}

#[async_trait]
pub trait Uploader {
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), ApiError>;
}
