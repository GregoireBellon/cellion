-- Your SQL goes here
CREATE TABLE students_groups(
    solution_id INTEGER NOT NULL REFERENCES solution,
    student_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, student_id) REFERENCES students(solution_id, label),
    FOREIGN KEY (solution_id, group_id) REFERENCES groups(solution_id, id),
    PRIMARY KEY (solution_id, group_id, student_id)
);