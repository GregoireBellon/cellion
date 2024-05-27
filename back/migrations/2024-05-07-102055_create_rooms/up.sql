-- Your SQL goes here
CREATE TABLE rooms(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    id TEXT NOT NULL,
    capacity INTEGER,
    name TEXT,
    PRIMARY KEY (solution_id, id)
)