-- Your SQL goes here
CREATE TABLE students(
    solution_id INTEGER NOT NULL REFERENCES solution,
    id TEXT NOT NULL,
    label TEXT,
    PRIMARY KEY (solution_id, id)
);