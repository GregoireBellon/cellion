-- Your SQL goes here
CREATE TABLE rooms(
    id TEXT PRIMARY KEY NOT NULL,
    solution_id NOT NULL REFERENCES solution,
    capacity INTEGER NOT NULL,
    name TEXT
)