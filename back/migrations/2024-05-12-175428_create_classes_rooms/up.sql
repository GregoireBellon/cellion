-- Your SQL goes here
CREATE TABLE classes_rooms(
    solution_id INTEGER NOT NULL REFERENCES solutions ON DELETE CASCADE,
    class_id TEXT NOT NULL,
    room_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (solution_id, room_id) REFERENCES rooms ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (solution_id, class_id, room_id)
);