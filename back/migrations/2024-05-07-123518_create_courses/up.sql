-- Your SQL goes here
CREATE TABLE courses (
    id TEXT PRIMARY KEY NOT NULL,
    solution_id INTEGER REFERENCES solutions,
    name TEXT
);