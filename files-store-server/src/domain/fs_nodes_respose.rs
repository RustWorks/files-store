use serde::Serialize;

use crate::domain::{FsNode, StoredFsNode};

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
