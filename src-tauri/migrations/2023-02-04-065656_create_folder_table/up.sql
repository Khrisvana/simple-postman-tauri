-- Your SQL goes here

CREATE TABLE IF NOT EXISTS folders (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    order_number INTEGER NOT NULL DEFAULT 1,
    parent_id INTEGER
)