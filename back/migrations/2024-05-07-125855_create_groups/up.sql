-- Your SQL goes here
CREATE TABLE groups (
    solution_id INTEGER NOT NULL REFERENCES solutions,
    id TEXT NOT NULL,
    PRIMARY KEY (solution_id, id)
);