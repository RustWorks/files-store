use uuid::Uuid;

use crate::{FsNodeMetadata, FsNodeType};

#[derive(Debug)]
pub struct CreateFsNode {
    pub uuid: Uuid,
    pub parent_id: i64,
    pub node_type: FsNodeType,
    pub name: String,
    pub metadata: FsNodeMetadata,
}

impl CreateFsNode {
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
