-- Your SQL goes here
CREATE TABLE teachers(
    solution_id INTEGER NOT NULL REFERENCES solution,
    name TEXT NOT NULL,
    department TEXT,
    PRIMARY KEY (solution_id, name)
);