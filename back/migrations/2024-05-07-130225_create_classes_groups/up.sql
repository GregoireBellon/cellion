-- Your SQL goes here
CREATE TABLE classes_groups(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    class_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes(solution_id, id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (solution_id, group_id) REFERENCES groups(solution_id, id) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (solution_id, class_id, group_id)
);