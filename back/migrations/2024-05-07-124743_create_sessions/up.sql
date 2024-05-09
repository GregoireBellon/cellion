-- Your SQL goes here
CREATE TABLE sessions (
    solution_id INTEGER NOT NULL REFERENCES solutions,
    uuid TEXT UNIQUE NOT NULL,
    class_id TEXT NOT NULL REFERENCES classes,
    rank INTEGER NOT NULL,
    starting_date TIMESTAMP NOT NULL,
    PRIMARY KEY (solution_id, class_id, rank)
);