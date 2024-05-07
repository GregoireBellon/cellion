CREATE TABLE solutions (
    id INTEGER PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT now
);