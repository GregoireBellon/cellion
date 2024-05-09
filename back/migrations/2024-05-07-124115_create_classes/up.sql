-- Your SQL goes here
CREATE TABLE classes (
    solution_id INTEGER NOT NULL REFERENCES solutions,
    id TEXT NOT NULL,
    part_id TEXT NOT NULL REFERENCES parts,
    PRIMARY KEY (solution_id, id)
);