use serde::Serialize;

use crate::FsNode;

#[derive(Debug, Serialize)]
pub struct FsNodesRespose {
    parent: FsNode,
    childrens: Vec<FsNode>,
    ancestors: Vec<FsNode>,
}

impl FsNodesRespose {
    pub fn new(parent: FsNode, childrens: Vec<FsNode>, ancestors: Vec<FsNode>) -> Self {
        Self {
            parent,
            childrens,
            ancestors,
        }
    }
}
