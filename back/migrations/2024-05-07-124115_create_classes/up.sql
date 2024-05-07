-- Your SQL goes here
CREATE TABLE classes (
    id TEXT PRIMARY KEY NOT NULL,
    part_id TEXT NOT NULL REFERENCES parts
);