-- Your SQL goes here

CREATE TABLE IF NOT EXISTS requests (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    url VARCHAR,
    method VARCHAR,
    order_number INTEGER NOT NULL DEFAULT 1,
    parent_id INTEGER
)