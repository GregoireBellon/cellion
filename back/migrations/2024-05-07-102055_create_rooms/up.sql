-- Your SQL goes here
CREATE TABLE rooms(
    solution_id INTEGER NOT NULL REFERENCES solutions,
    id TEXT NOT NULL,
    capacity INTEGER,
    name TEXT,
    PRIMARY KEY (solution_id, id)
)