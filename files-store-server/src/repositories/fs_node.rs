use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;
use std::string::ToString;
use strum::EnumString;
use uuid::Uuid;

#[derive(Debug, PartialEq, EnumString, strum::ToString, Deserialize, Serialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum FsNodeType {
    File,
    Directory,
    Root,
    Bin,
    Thumbnail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum FsNodeMetadata {
    File {
        hash: String,
        content_type: String,
        size: i64,
    },
    Directory,
    Bin,
    Thumbnail {
        content_type: String,
        size: i64,
        width: i64,
        heigth: i64,
    },
}

impl FsNodeMetadata {
    pub fn new_file(hash: String, content_type: String, size: i64) -> Self {
        Self::File {
            hash,
            content_type,
            size,
        }
    }

    pub fn new_thumbnail(content_type: String, size: i64, width: i64, heigth: i64) -> Self {
        Self::Thumbnail {
            content_type,
            size,
            width,
            heigth,
        }
    }
}

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

#[derive(Debug)]
pub struct CreateStoredFsNode {
    pub uuid: Uuid,
    pub parent_id: i64,
    pub node_type: FsNodeType,
    pub name: String,
    pub metadata: FsNodeMetadata,
}

impl CreateStoredFsNode {
    pub fn new(
        uuid: Uuid,
        parent_id: i64,
        node_type: FsNodeType,
        name: String,
        metadata: FsNodeMetadata,
    ) -> Self {
        Self {
            uuid,
            parent_id,
            node_type,
            name,
            metadata,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FsNodesRespose {
    parent: FsNode,
    childrens: Vec<FsNode>,
    ancestors: Vec<FsNode>,
}

impl FsNodesRespose {
    pub fn new(
        parent: StoredFsNode,
        childrens: Vec<StoredFsNode>,
        ancestors: Vec<StoredFsNode>,
    ) -> Self {
        let childrens = childrens.into_iter().map(FsNode::from).collect();
        let ancestors = ancestors.into_iter().map(FsNode::from).collect();
        let parent = parent.into();
        Self {
            parent,
            childrens,
            ancestors,
        }
    }
}
