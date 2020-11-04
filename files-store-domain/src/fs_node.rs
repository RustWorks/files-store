use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::FsNodeMetadata;

#[derive(Debug, Deserialize, Serialize)]
pub struct FsNode {
    pub uuid: Uuid,
    pub node_type: String,
    pub name: String,
    pub metadata: FsNodeMetadata,
    pub user_uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
