use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::ApiError;

#[async_trait]
pub trait Storage {
    async fn get_uploader(
        &self,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<Box<dyn Uploader>, ApiError>;

    async fn get_file(&self, uuid: &Uuid, user_uuid: &Uuid) -> Result<tokio::fs::File, ApiError>;

    async fn remove_file(&self, uuid: &Uuid, user_uuid: &Uuid) -> Result<(), ApiError>;
}

#[async_trait]
pub trait Uploader {
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), ApiError>;
}
