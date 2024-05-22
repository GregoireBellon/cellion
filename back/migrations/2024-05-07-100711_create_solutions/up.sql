CREATE TABLE solutions (
    id INTEGER PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL,
    slot_duration INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT now
);