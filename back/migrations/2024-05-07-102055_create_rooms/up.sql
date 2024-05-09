-- Your SQL goes here
CREATE TABLE rooms(
    id TEXT PRIMARY KEY NOT NULL,
    solution_id INTEGER NOT NULL REFERENCES solutions,
    capacity INTEGER,
    name TEXT
)