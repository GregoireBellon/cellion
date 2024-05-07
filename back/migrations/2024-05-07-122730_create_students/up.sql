-- Your SQL goes here
CREATE TABLE students(
    id TEXT NOT NULL PRIMARY KEY,
    solution_id INTEGER NOT NULL REFERENCES solution,
    name TEXT,
    label TEXT
);