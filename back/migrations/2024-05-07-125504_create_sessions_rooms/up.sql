-- Your SQL goes here
CREATE TABLE sessions_rooms (
    session_id INTEGER NOT NULL,
    room_id TEXT NOT NULL,
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    PRIMARY KEY (session_id, room_id, solution_id),
    FOREIGN KEY (session_id) REFERENCES sessions (id) ON DELETE CASCADE,
    FOREIGN KEY (solution_id, room_id) REFERENCES rooms(solution_id, id) ON DELETE CASCADE ON UPDATE CASCADE
);