-- Your SQL goes here
CREATE TABLE sessions_rooms (
    solution_id INTEGER NOT NULL,
    class_id TEXT NOT NULL,
    session_rank INTEGER NOT NULL,
    room_id TEXT NOT NULL,
    PRIMARY KEY (solution_id, class_id, session_rank, room_id),
    FOREIGN KEY (solution_id, class_id, session_rank) REFERENCES sessions (solution_id, class_id, rank),
    FOREIGN KEY (solution_id, room_id) REFERENCES rooms
);