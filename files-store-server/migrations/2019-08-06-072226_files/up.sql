-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  uuid UUID PRIMARY KEY NOT NULL,
  login TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE fs_nodes (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  uuid UUID NOT NULL DEFAULT gen_random_uuid(),
  parent_id BIGINT,
  node_type TEXT NOT NULL,
  name TEXT NOT NULL,
  metadata JSONB NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP,
  user_uuid UUID NOT NULL REFERENCES users(uuid),

  FOREIGN KEY (parent_id) REFERENCES fs_nodes(id) ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE INDEX fs_nodes_parent_id_index ON fs_nodes USING btree (parent_id);
CREATE INDEX fs_nodes_uuid_index ON fs_nodes USING btree (uuid);

CREATE TABLE fs_nodes_tree_paths (
  ancestor_id BIGINT NOT NULL,
  descendant_id BIGINT NOT NULL,
  depth INTEGER NOT NULL,

  PRIMARY KEY (ancestor_id, descendant_id),
  FOREIGN KEY (ancestor_id) REFERENCES fs_nodes(id),
  FOREIGN KEY (descendant_id) REFERENCES fs_nodes(id)
);

CREATE INDEX fs_nodes_tree_paths_ancestor_id_index ON fs_nodes_tree_paths USING btree (ancestor_id);
CREATE INDEX fs_nodes_tree_paths_descendant_id_index ON fs_nodes_tree_paths USING btree (descendant_id);

CREATE OR REPLACE FUNCTION after_change_fs_node() RETURNS TRIGGER LANGUAGE PLPGSQL AS $$
BEGIN
  IF (TG_OP = 'INSERT') THEN
    INSERT INTO fs_nodes_tree_paths(ancestor_id, descendant_id, depth)
    SELECT ancestor_id, NEW.id, depth + 1
    FROM fs_nodes_tree_paths WHERE descendant_id = NEW.parent_id
    UNION ALL SELECT NEW.id, NEW.id, 0;
  END IF;
  RETURN NULL;
END;
$$;

CREATE TRIGGER after_change_fs_node AFTER INSERT OR UPDATE
  ON fs_nodes FOR EACH ROW EXECUTE PROCEDURE after_change_fs_node();
