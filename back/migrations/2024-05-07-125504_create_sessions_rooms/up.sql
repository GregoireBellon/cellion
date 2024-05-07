-- Your SQL goes here
CREATE TABLE sessions_rooms (
    class_id TEXT NOT NULL,
    session_rank INTEGER NOT NULL,
    room_id TEXT NOT NULL,
    PRIMARY KEY (class_id, session_rank, room_id),
    FOREIGN KEY (class_id, session_rank) REFERENCES sessions (class_id, rank),
    FOREIGN KEY (room_id) REFERENCES rooms
);