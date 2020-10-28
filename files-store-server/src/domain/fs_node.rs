use chrono::NaiveDateTime;
use serde::Serialize;
use std::string::ToString;
use uuid::Uuid;

use crate::domain::{FsNodeMetadata, StoredFsNode};

#[derive(Debug, Serialize)]
pub struct FsNode {
    pub uuid: Uuid,
    pub node_type: String,
    pub name: String,
    pub metadata: FsNodeMetadata,
    pub user_uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StoredFsNode> for FsNode {
    fn from(stored_fs_node: StoredFsNode) -> Self {
        Self {
            uuid: stored_fs_node.uuid,
            node_type: stored_fs_node.node_type.to_string(),
            name: stored_fs_node.name,
            metadata: stored_fs_node.metadata.0,
            user_uuid: stored_fs_node.user_uuid,
            created_at: stored_fs_node.created_at,
            updated_at: stored_fs_node.updated_at,
        }
    }
}
