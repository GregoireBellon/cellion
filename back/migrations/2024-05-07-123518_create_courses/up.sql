-- Your SQL goes here
CREATE TABLE courses (
    solution_id INTEGER REFERENCES solutions,
    id TEXT NOT NULL,
    name TEXT,
    PRIMARY KEY (solution_id, id)
);