-- Your SQL goes here
CREATE TABLE classes_teachers(
    solution_id INTEGER NOT NULL REFERENCES solutions,
    class_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes,
    FOREIGN KEY (solution_id, teacher_id) REFERENCES teachers,
    PRIMARY KEY (solution_id, class_id, teacher_id)
);