-- This file should undo anything in `up.sql`

DROP INDEX fs_nodes_tree_paths_ancestor_id_index;
DROP INDEX fs_nodes_tree_paths_descendant_id_index;
DROP TABLE fs_nodes_tree_paths;

DROP TRIGGER IF EXISTS after_change_fs_node ON fs_nodes;
DROP FUNCTION IF EXISTS after_change_fs_node();

DROP INDEX fs_nodes_parent_id_index;
DROP INDEX fs_nodes_uuid_index;
DROP TABLE fs_nodes;
DROP TABLE users;
