-- Your SQL goes here
CREATE TABLE classes_groups(
    class_id TEXT NOT NULL REFERENCES classes,
    group_id TEXT NOT NULL REFERENCES groups,
    PRIMARY KEY (class_id, group_id)
);