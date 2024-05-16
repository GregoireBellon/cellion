-- Your SQL goes here
CREATE TABLE sessions (
    id INTEGER NOT NULL PRIMARY KEY,
    solution_id INTEGER NOT NULL REFERENCES solutions,
    uuid TEXT UNIQUE NOT NULL,
    class_id TEXT NOT NULL,
    rank INTEGER NOT NULL,
    starting_date TIMESTAMP NOT NULL,
    UNIQUE (solution_id, class_id, rank),
    FOREIGN KEY (solution_id, class_id) REFERENCES classes(solution_id, id)
);