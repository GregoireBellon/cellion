-- Your SQL goes here
CREATE TABLE classes_groups(
    solution_id INTEGER NOT NULL REFERENCES solutions,
    class_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes,
    FOREIGN KEY (solution_id, group_id) REFERENCES groups,
    PRIMARY KEY (solution_id, class_id, group_id)
);