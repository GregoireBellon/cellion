-- Your SQL goes here
CREATE TABLE courses (
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    id TEXT NOT NULL,
    name TEXT,
    PRIMARY KEY (solution_id, id)
);