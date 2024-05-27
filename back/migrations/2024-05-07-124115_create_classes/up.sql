-- Your SQL goes here
CREATE TABLE classes (
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    id TEXT NOT NULL,
    part_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, part_id) REFERENCES parts(solution_id, id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (solution_id, id)
);