-- Your SQL goes here
CREATE TABLE teachers(
    name TEXT PRIMARY KEY NOT NULL,
    solution_id INTEGER NOT NULL REFERENCES solution,
    department TEXT
);