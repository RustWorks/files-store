use async_trait::async_trait;
use sqlx::postgres::PgQueryAs;
use sqlx::{query_as, PgConnection};
use uuid::Uuid;

use crate::auth::User;
use crate::repositories::{CreateStoredFsNode, FsNodeType, RepositoryError, StoredFsNode};

#[async_trait]
pub trait FsNodeStore {
    async fn insert(
        &mut self,
        create_stored_fs_node: &CreateStoredFsNode,
        user: &User,
    ) -> Result<StoredFsNode, RepositoryError>;

    async fn find_fs_node_by_name(
        &mut self,
        parent_id: i64,
        name: &str,
        user: &User,
    ) -> Result<Option<StoredFsNode>, RepositoryError>;

    async fn find_root_fs_node(
        &mut self,
        user: &User,
        fs_node_type: FsNodeType,
    ) -> Result<StoredFsNode, RepositoryError>;

    async fn find_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        fs_node_type: FsNodeType,
        user: &User,
    ) -> Result<StoredFsNode, RepositoryError>;

    async fn find_fs_nodes_by_parent_id(
        &mut self,
        parent_id: i64,
        user: &User,
    ) -> Result<Vec<StoredFsNode>, RepositoryError>;

    async fn find_fs_nodes_ancestor_by_id(
        &mut self,
        id: i64,
        user: &User,
    ) -> Result<Vec<StoredFsNode>, RepositoryError>;
}

#[async_trait]
impl FsNodeStore for PgConnection {
    async fn insert(
        &mut self,
        create_stored_fs_node: &CreateStoredFsNode,
        user: &User,
    ) -> Result<StoredFsNode, RepositoryError> {
        let stored_fs_node = query_as(
            r#"
            INSERT INTO fs_nodes (
                node_type,
                parent_id,
                name,
                metadata,
                user_uuid
            )
            VALUES ($1, $2, $3, $4, $5) RETURNING *
            "#,
        )
        .bind(&create_stored_fs_node.node_type.to_string())
        .bind(create_stored_fs_node.parent_id)
        .bind(&create_stored_fs_node.name)
        .bind(&create_stored_fs_node.metadata)
        .bind(user.uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_root_fs_node(
        &mut self,
        user: &User,
        fs_node_type: FsNodeType,
    ) -> Result<StoredFsNode, RepositoryError> {
        let stored_fs_node = query_as(
            "SELECT * FROM fs_nodes WHERE parent_id IS NULL AND node_type = $1 AND user_uuid = $2",
        )
        .bind(user.uuid)
        .bind(fs_node_type.to_string())
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        fs_node_type: FsNodeType,
        user: &User,
    ) -> Result<StoredFsNode, RepositoryError> {
        let stored_fs_node = query_as(
            "SELECT * FROM fs_nodes WHERE uuid = $1 AND node_type = $2 AND user_uuid = $3",
        )
        .bind(uuid)
        .bind(fs_node_type.to_string())
        .bind(user.uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_fs_nodes_by_parent_id(
        &mut self,
        parent_id: i64,
        user: &User,
    ) -> Result<Vec<StoredFsNode>, RepositoryError> {
        let fs_nodes = query_as(
            r#"
            SELECT d.*
                -- concat(repeat('-', p.depth), d.name) AS tree,
                -- p.depth,
                -- array_to_string(array_agg(crumbs.ancestor_id::CHARACTER VARYING ORDER BY crumbs.ancestor_id),'/','*') breadcrumbs
            FROM fs_nodes AS d
                JOIN fs_nodes_tree_paths AS p
                    ON d.id = p.descendant_id
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND d.is_deleted = false
                -- AND p.depth < 2
                AND p.depth = 1
                AND user_uuid = $2
            GROUP BY d.id, p.depth
            ORDER BY d.id ASC
            "#,
        )
        .bind(parent_id)
        .bind(user.uuid)
        .fetch_all(self)
        .await?;
        Ok(fs_nodes)
    }

    async fn find_fs_nodes_ancestor_by_id(
        &mut self,
        id: i64,
        user: &User,
    ) -> Result<Vec<StoredFsNode>, RepositoryError> {
        let fs_nodes = query_as(
            r#"
            SELECT fs_nodes.* --, fs_nodes_tree_paths.*
            FROM fs_nodes
            JOIN fs_nodes_tree_paths ON fs_nodes.id = fs_nodes_tree_paths.ancestor_id
            WHERE fs_nodes_tree_paths.descendant_id = $1
                AND user_uuid = $2
            ORDER BY fs_nodes_tree_paths.depth DESC
            "#,
        )
        .bind(id)
        .bind(user.uuid)
        .fetch_all(self)
        .await?;
        Ok(fs_nodes)
    }

    async fn find_fs_node_by_name(
        &mut self,
        parent_id: i64,
        name: &str,
        user: &User,
    ) -> Result<Option<StoredFsNode>, RepositoryError> {
        let fs_node = query_as(
            r#"
            SELECT d.*
            FROM fs_nodes AS d
                JOIN fs_nodes_tree_paths AS p
                    ON d.id = p.descendant_id
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND d.is_deleted = false
                AND d.name = $2
                AND p.depth = 1
                AND user_uuid = $3
            GROUP BY d.id, p.depth
            "#,
        )
        .bind(parent_id)
        .bind(name)
        .bind(user.uuid)
        .fetch_optional(self)
        .await?;
        Ok(fs_node)
    }
}
