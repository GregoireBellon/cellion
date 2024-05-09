-- Your SQL goes here
CREATE TABLE sessions_teachers (
    solution_id INTEGER NOT NULL REFERENCES solutions,
    class_id TEXT NOT NULL,
    session_rank INTEGER NOT NULL,
    teacher_id TEXT NOT NULL,
    PRIMARY KEY (solution_id, class_id, session_rank, teacher_id),
    FOREIGN KEY (solution_id, class_id, session_rank) REFERENCES sessions,
    FOREIGN KEY (solution_id, teacher_id) REFERENCES teachers
);