-- Your SQL goes here
CREATE TABLE sessions_teachers (
    class_id TEXT NOT NULL,
    session_rank INTEGER NOT NULL,
    teacher_id TEXT NOT NULL,
    PRIMARY KEY (class_id, session_rank, teacher_id),
    FOREIGN KEY (class_id, session_rank) REFERENCES sessions (class_id, rank),
    FOREIGN KEY (teacher_id) REFERENCES teachers
);