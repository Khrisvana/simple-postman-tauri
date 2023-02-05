-- Your SQL goes here

CREATE TABLE IF NOT EXISTS requests (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR,
    url VARCHAR,
    method VARCHAR NOT NULL DEFAULT 'GET',
    folder_id INTEGER
)