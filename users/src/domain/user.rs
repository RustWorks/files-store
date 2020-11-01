use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub login: String,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(login: String, email: String, password: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            login,
            email: email.trim().to_lowercase(),
            password,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
