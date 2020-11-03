use chrono::NaiveDateTime;
use files_store_domain::{FsNode, FsNodeMetadata, FsNodesRespose};
use sqlx::types::Json;
use sqlx::FromRow;
use uuid::Uuid;

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

pub fn create_fs_nodes_respose(
    parent: StoredFsNode,
    childrens: Vec<StoredFsNode>,
    ancestors: Vec<StoredFsNode>,
) -> FsNodesRespose {
    let childrens = childrens.into_iter().map(FsNode::from).collect();
    let ancestors = ancestors.into_iter().map(FsNode::from).collect();
    FsNodesRespose::new(parent.into(), childrens, ancestors)
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
