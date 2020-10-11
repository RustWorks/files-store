use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value;
use sqlx::FromRow;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub enum FsNodeType {
    File,
    Directory,
}

impl Display for FsNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsNodeType::File => write!(f, "file"),
            FsNodeType::Directory => write!(f, "directory"),
        }
    }
}

impl FsNodeType {
    #[allow(dead_code)]
    pub fn parse(text: &str) -> Self {
        match text {
            "file" => FsNodeType::File,
            "directory" => FsNodeType::Directory,
            _ => panic!("FsNodeType parsing error: {}", text),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct StoredFsNode {
    pub id: i64,
    pub uuid: Uuid,
    pub parent_id: Option<i64>,
    pub node_type: String,
    pub name: String,
    pub metadata: Value,
    pub is_deleted: bool,
    pub user_uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct FsNode {
    pub uuid: Uuid,
    pub node_type: String,
    pub name: String,
    pub metadata: Value,
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
            metadata: stored_fs_node.metadata,
            user_uuid: stored_fs_node.user_uuid,
            created_at: stored_fs_node.created_at,
            updated_at: stored_fs_node.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct CreateStoredFsNode {
    pub parent_id: i64,
    pub node_type: FsNodeType,
    pub name: String,
    pub metadata: Value,
}

impl CreateStoredFsNode {
    pub fn new(parent_id: i64, node_type: FsNodeType, name: String, metadata: Value) -> Self {
        Self {
            parent_id,
            node_type,
            name,
            metadata,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileFsNodeMetaData {
    pub hash: String,
    pub content_type: String,
    pub size: i64,
}

impl FileFsNodeMetaData {
    pub fn new(hash: String, content_type: String, size: i64) -> Self {
        Self {
            hash,
            content_type,
            size,
        }
    }
}