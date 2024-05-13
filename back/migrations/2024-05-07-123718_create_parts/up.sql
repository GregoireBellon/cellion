-- Your SQL goes here
CREATE TABLE parts (
    solution_id INTEGER NOT NULL REFERENCES solutions,
    id TEXT NOT NULL,
    course_id TEXT NOT NULL REFERENCES courses,
    session_lenght INTEGER NOT NULL,
    session_teachers INTEGER,
    session_rooms TEXT,
    session_length INTEGER,
    label TEXT,
    max_head_count INTEGER,
    nr_session INTEGER,
    PRIMARY KEY (solution_id, id)
);