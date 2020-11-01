use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{query, query_as, types::Json, Done, Error, PgConnection};
use uuid::Uuid;

use crate::domain::{CreateStoredFsNode, FsNodeMetadata, FsNodeType, StoredFsNode};
use crate::repositories::FsNodeStore;

#[async_trait]
impl FsNodeStore for PgConnection {
    async fn insert_fs_node(
        &mut self,
        create_stored_fs_node: CreateStoredFsNode,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error> {
        let stored_fs_node = query_as(
            r#"
            INSERT INTO fs_nodes (
                uuid,
                node_type,
                parent_id,
                name,
                metadata,
                user_uuid
            )
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
            "#,
        )
        .bind(create_stored_fs_node.uuid)
        .bind(&create_stored_fs_node.node_type.to_string())
        .bind(create_stored_fs_node.parent_id)
        .bind(&create_stored_fs_node.name)
        .bind(Json(create_stored_fs_node.metadata))
        .bind(user_uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn insert_root_fs_node(
        &mut self,
        fs_node_type: &FsNodeType,
        name: &str,
        metadata: &FsNodeMetadata,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error> {
        let stored_fs_node = query_as(
            r#"
            INSERT INTO fs_nodes (
                node_type,
                name,
                metadata,
                user_uuid
            )
            VALUES ($1, $2, $3, $4) RETURNING *
            "#,
        )
        .bind(fs_node_type.to_string())
        .bind(name)
        .bind(Json(metadata))
        .bind(user_uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_root_fs_node(
        &mut self,
        fs_node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error> {
        let stored_fs_node = query_as(
            r#"
            SELECT fs_nodes.*
            FROM fs_nodes
            WHERE parent_id IS NULL
                AND node_type = $1
                AND user_uuid = $2
        "#,
        )
        .bind(fs_node_type.to_string())
        .bind(user_uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        fs_node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error> {
        let stored_fs_node = query_as(
            r#"
            SELECT fs_nodes.*
            FROM fs_nodes
            WHERE uuid = $1
                AND (node_type = $2 OR node_type = 'root')
                AND user_uuid = $3
        "#,
        )
        .bind(uuid)
        .bind(fs_node_type.to_string())
        .bind(user_uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }
    async fn find_any_fs_node_by_uuid(
        &mut self,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<StoredFsNode, Error> {
        let stored_fs_node = query_as(
            r#"
            SELECT fs_nodes.*
            FROM fs_nodes
            WHERE uuid = $1
                AND user_uuid = $2
        "#,
        )
        .bind(uuid)
        .bind(user_uuid)
        .fetch_one(self)
        .await?;
        Ok(stored_fs_node)
    }

    async fn find_fs_nodes_by_parent_id(
        &mut self,
        parent_id: i64,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error> {
        let fs_nodes = query_as(
            r#"
            SELECT d.*
            FROM fs_nodes AS d
                JOIN fs_nodes_tree_paths AS p
                    ON d.id = p.descendant_id
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND p.depth = 1
                AND user_uuid = $2
            GROUP BY d.id, p.depth
            ORDER BY node_type ASC, name ASC
            "#,
        )
        .bind(parent_id)
        .bind(user_uuid)
        .fetch_all(self)
        .await?;
        Ok(fs_nodes)
    }

    async fn find_fs_nodes_ancestor_by_id(
        &mut self,
        id: i64,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error> {
        let fs_nodes = query_as(
            r#"
            SELECT fs_nodes.*
            FROM fs_nodes
            JOIN fs_nodes_tree_paths ON fs_nodes.id = fs_nodes_tree_paths.ancestor_id
            WHERE fs_nodes_tree_paths.descendant_id = $1
                AND user_uuid = $2
            ORDER BY fs_nodes_tree_paths.depth DESC
            "#,
        )
        .bind(id)
        .bind(user_uuid)
        .fetch_all(self)
        .await?;
        Ok(fs_nodes)
    }

    async fn find_fs_node_by_name(
        &mut self,
        parent_id: i64,
        name: &str,
        user_uuid: &Uuid,
    ) -> Result<Option<StoredFsNode>, Error> {
        let fs_node = query_as(
            r#"
            SELECT d.*
            FROM fs_nodes AS d
                JOIN fs_nodes_tree_paths AS p
                    ON d.id = p.descendant_id
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND d.name = $2
                AND p.depth = 1
                AND user_uuid = $3
            GROUP BY d.id, p.depth
            "#,
        )
        .bind(parent_id)
        .bind(name)
        .bind(user_uuid)
        .fetch_optional(self)
        .await?;
        Ok(fs_node)
    }

    async fn find_fs_node_thumbnail_by_uuid(
        &mut self,
        parent_id: i64,
        user_uuid: &Uuid,
    ) -> Result<Option<StoredFsNode>, Error> {
        let fs_node = query_as(
            r#"
            SELECT d.*
            FROM fs_nodes AS d
                JOIN fs_nodes_tree_paths AS p
                    ON d.id = p.descendant_id
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND p.depth = 1
                AND user_uuid = $2
                AND d.node_type = 'thumbnail'
            GROUP BY d.id, p.depth
            "#,
        )
        .bind(parent_id)
        .bind(user_uuid)
        .fetch_optional(self)
        .await?;
        Ok(fs_node)
    }

    async fn update_deleted_at_fs_node(&mut self, id: i64, user_uuid: &Uuid) -> Result<u64, Error> {
        let updated = query(
            r#"
            UPDATE fs_nodes AS d
            SET deleted_at = NOW()
            FROM fs_nodes_tree_paths AS p 
                JOIN fs_nodes_tree_paths AS crumbs
                    ON crumbs.descendant_id = p.descendant_id
            WHERE p.ancestor_id = $1
                AND d.id = p.descendant_id
                AND d.user_uuid = $2
        "#,
        )
        .bind(id)
        .bind(user_uuid)
        .execute(self)
        .await?;
        Ok(updated.rows_affected())
    }

    async fn delete_fs_node(&mut self, id: i64) -> Result<u64, Error> {
        let deleted = query(
            r#"
            DELETE FROM fs_nodes_tree_paths
            WHERE descendant_id IN (
                SELECT descendant_id
                FROM fs_nodes_tree_paths
                WHERE ancestor_id = $1
            )
        "#,
        )
        .bind(id)
        .execute(self)
        .await?;
        Ok(deleted.rows_affected())
    }

    async fn move_fs_node_update_parent_id(&mut self, src: i64, dest: i64) -> Result<u64, Error> {
        let updated = query(
            r#"
            UPDATE fs_nodes SET parent_id = $2 WHERE id = $1
        "#,
        )
        .bind(src)
        .bind(dest)
        .execute(self)
        .await?;
        Ok(updated.rows_affected())
    }

    async fn move_fs_node_disconnect(&mut self, src: i64) -> Result<u64, Error> {
        let updated = query(
            r#"
            DELETE FROM fs_nodes_tree_paths
            WHERE descendant_id IN (
                SELECT descendant_id
                FROM fs_nodes_tree_paths
                WHERE ancestor_id = $1
            )
            AND ancestor_id IN (
                SELECT ancestor_id
                FROM fs_nodes_tree_paths
                WHERE descendant_id = $1
                    AND ancestor_id != descendant_id
            )
        "#,
        )
        .bind(src)
        .execute(self)
        .await?;
        Ok(updated.rows_affected())
    }

    async fn move_fs_node_update_ancestors(&mut self, src: i64, dest: i64) -> Result<u64, Error> {
        let updated = query(
            r#"
            INSERT INTO fs_nodes_tree_paths (ancestor_id, descendant_id, depth)
            SELECT supertree.ancestor_id, subtree.descendant_id, supertree.depth + subtree.depth + 1
            FROM fs_nodes_tree_paths AS supertree
            CROSS JOIN fs_nodes_tree_paths AS subtree
            WHERE supertree.descendant_id = $2
            AND subtree.ancestor_id = $1
        "#,
        )
        .bind(src)
        .bind(dest)
        .execute(self)
        .await?;
        Ok(updated.rows_affected())
    }

    async fn find_deleted_fs_nodes(
        &mut self,
        date: &NaiveDateTime,
        node_type: &FsNodeType,
        user_uuid: &Uuid,
    ) -> Result<Vec<StoredFsNode>, Error> {
        let stored_fs_node = query_as(
            r#"
            SELECT *
            FROM fs_nodes
            WHERE node_type = $1
                AND deleted_at < $2
                AND user_uuid = $3
        "#,
        )
        .bind(node_type.to_string())
        .bind(date)
        .bind(user_uuid)
        .fetch_all(self)
        .await?;
        Ok(stored_fs_node)
    }
}
