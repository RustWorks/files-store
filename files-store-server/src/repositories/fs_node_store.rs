use async_trait::async_trait;
use chrono::NaiveDateTime;
use files_store_domain::{CreateFsNode, FsNodeMetadata, FsNodeType};
use sqlx::Error;
use uuid::Uuid;

use crate::repositories::StoredFsNode;

#[async_trait]
pub trait FsNodeStore {
    async fn insert_fs_node(
        &mut self,
        create_stored_fs_node: CreateFsNode,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error>;

    async fn insert_root_fs_node(
        &mut self,
        fs_node_type: &FsNodeType,
        name: &str,
        metadata: &FsNodeMetadata,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error>;

    async fn find_fs_node_by_name(
        &mut self,
        parent_id: i64,
        name: &str,
        user_uuid: &Uuid,
    ) -> Result<Option<StoredFsNode>, Error>;

    async fn find_fs_node_thumbnail_by_uuid(
        &mut self,
        parent_id: i64,
        user_uuid: &Uuid,
    ) -> Result<Option<StoredFsNode>, Error>;

    async fn find_root_fs_node(
        &mut self,
        fs_node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error>;

    async fn find_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        fs_node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error>;

    async fn find_any_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error>;

    async fn find_fs_nodes_by_parent_id(
        &mut self,
        parent_id: i64,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error>;

    async fn find_fs_nodes_ancestor_by_id(
        &mut self,
        id: i64,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error>;

    async fn update_deleted_at_fs_node(&mut self, id: i64, user_uuid: &Uuid) -> Result<u64, Error>;

    async fn delete_fs_node(&mut self, id: i64) -> Result<u64, Error>;

    async fn move_fs_node_update_parent_id(&mut self, src: i64, dest: i64) -> Result<u64, Error>;

    async fn move_fs_node_disconnect(&mut self, src: i64) -> Result<u64, Error>;

    async fn move_fs_node_update_ancestors(&mut self, src: i64, dest: i64) -> Result<u64, Error>;

    async fn find_deleted_fs_nodes(
        &mut self,
        date: &NaiveDateTime,
        node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error>;
}
