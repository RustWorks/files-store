use chrono::{NaiveDateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, new)]
pub struct UploadFile {
    pub path: String,
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UploadedFile {
    pub uuid: Uuid,
    pub owner: Uuid,
    pub path: String,
    pub name: String,
    pub directory: String,
    pub storage: String,
    pub hash: String,
    pub size: i64,
    pub content_type: String,
    pub metadata: Option<Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UploadedFile {
    pub fn new(path: String, name: String, directory: String, storage: String, hash: String, size: i64, content_type: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            owner: Uuid::new_v4(),
            path,
            name,
            directory,
            storage,
            hash,
            size,
            content_type,
            metadata: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
