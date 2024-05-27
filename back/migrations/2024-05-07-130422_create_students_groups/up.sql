-- Your SQL goes here
CREATE TABLE students_groups(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    student_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, student_id) REFERENCES students ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (solution_id, group_id) REFERENCES groups ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (solution_id, group_id, student_id)
);