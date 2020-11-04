use files_store_domain::{CreateFsNodeDirectory, FsNode};
use reqwest::{Client, StatusCode};

use crate::errors::AppError;

pub struct Api {
    client: Client,
    token: String,
    host: String,
}

impl Api {
    pub fn new(token: String, host: String) -> Self {
        Self {
            client: Client::new(),
            token,
            host,
        }
    }

    pub async fn create_directory(
        &self,
        create_fs_node_directory: &CreateFsNodeDirectory,
    ) -> Result<FsNode, AppError> {
        let url = format!("{}/api/fs/directories", self.host);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&create_fs_node_directory)
            .send()
            .await?;
        dbg!(&response);
        let status = response.status();
        if status == StatusCode::OK {
            let data = response.json().await?;
            Ok(data)
        } else {
            Err(AppError::Api)
        }
    }
}
