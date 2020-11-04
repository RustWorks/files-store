use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFsNodeDirectory {
    pub name: String,
    pub parent_uuid: Option<Uuid>,
}

impl CreateFsNodeDirectory {
    pub fn new(name: String, parent_uuid: Option<Uuid>) -> Self {
        Self { name, parent_uuid }
    }
}
