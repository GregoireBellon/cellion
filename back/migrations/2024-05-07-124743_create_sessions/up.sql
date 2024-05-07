-- Your SQL goes here
CREATE TABLE sessions (
    class_id TEXT NOT NULL REFERENCES classes,
    rank INTEGER NOT NULL,
    starting_date DATETIME NOT NULL,
    PRIMARY KEY (class_id, rank)
);