-- Your SQL goes here
CREATE TABLE parts (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL REFERENCES courses,
    session_teachers INTEGER,
    session_rooms TEXT,
    session_length INTEGER,
    label TEXT,
    max_head_count INTEGER,
    nr_session INTEGER
);