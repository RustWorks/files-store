use chrono::NaiveDateTime;
use sqlx::types::Json;
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::FsNodeMetadata;

#[derive(Debug, Clone, FromRow)]
pub struct StoredFsNode {
    pub id: i64,
    pub uuid: Uuid,
    pub parent_id: Option<i64>,
    pub node_type: String,
    pub name: String,
    pub metadata: Json<FsNodeMetadata>,
    pub user_uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
