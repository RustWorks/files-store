use crate::errors::ApiError;

pub trait Storage {

}

#[derive(Debug, Clone)]
pub struct LocalStorage {
    pub local_storage_path: String
}

impl LocalStorage {
    pub async fn new(local_storage_path: &str) -> Result<Self, ApiError> {
        let path = async_std::path::Path::new(local_storage_path);
        if path.exists().await {
            let local_storage = Self {
                local_storage_path: local_storage_path.to_owned()
            };
            Ok(local_storage)
        } else {
            Err(ApiError::InternalServer)
        }
    }
}

impl Storage for LocalStorage {

}
