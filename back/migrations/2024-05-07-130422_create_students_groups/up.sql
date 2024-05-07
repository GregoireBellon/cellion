-- Your SQL goes here
CREATE TABLE students_groups(
    student_id TEXT NOT NULL REFERENCES students,
    group_id TEXT NOT NULL REFERENCES groups,
    PRIMARY KEY (student_id, group_id)
);