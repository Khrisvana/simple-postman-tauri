-- Your SQL goes here

CREATE TABLE IF NOT EXISTS folders (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    parent_id INTEGER
)