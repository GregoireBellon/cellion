-- Your SQL goes here
CREATE TABLE groups (
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    id TEXT NOT NULL,
    PRIMARY KEY (solution_id, id)
);