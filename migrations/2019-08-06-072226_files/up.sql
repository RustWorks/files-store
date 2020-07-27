-- Your SQL goes here

CREATE TABLE IF NOT EXISTS files (
    uuid UUID PRIMARY KEY NOT NULL,
    owner UUID NOT NULL,
    path TEXT NOT NULL,
    name TEXT NOT NULL,
    hash TEXT NOT NULL,
    size BIGINT NOT NULL DEFAULT 0,
    content_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
