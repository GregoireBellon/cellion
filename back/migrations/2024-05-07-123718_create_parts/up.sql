-- Your SQL goes here
CREATE TABLE parts (
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    session_length INTEGER NOT NULL,
    session_teachers INTEGER,
    session_rooms TEXT,
    label TEXT,
    max_head_count INTEGER,
    nr_session INTEGER,
    FOREIGN KEY (solution_id, course_id) REFERENCES courses(solution_id, id) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (solution_id, id)
);