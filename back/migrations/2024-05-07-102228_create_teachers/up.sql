-- Your SQL goes here
CREATE TABLE teachers(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    name TEXT NOT NULL,
    department TEXT,
    PRIMARY KEY (solution_id, name)
);