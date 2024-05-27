-- Your SQL goes here
CREATE TABLE classes_teachers(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    class_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (solution_id, teacher_id) REFERENCES teachers ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (solution_id, class_id, teacher_id)
);