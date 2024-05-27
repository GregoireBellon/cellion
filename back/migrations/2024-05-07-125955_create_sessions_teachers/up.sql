-- Your SQL goes here
CREATE TABLE sessions_teachers (
    session_id INTEGER NOT NULL REFERENCES sessions,
    teacher_id TEXT NOT NULL,
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    PRIMARY KEY (session_id, solution_id, teacher_id),
    FOREIGN KEY (solution_id, teacher_id) REFERENCES teachers ON UPDATE CASCADE ON DELETE CASCADE
);