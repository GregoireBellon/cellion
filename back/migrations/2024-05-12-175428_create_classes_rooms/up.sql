-- Your SQL goes here
CREATE TABLE classes_rooms(
    solution_id INTEGER NOT NULL REFERENCES solutions,
    class_id TEXT NOT NULL,
    room_id TEXT NOT NULL,
    FOREIGN KEY (solution_id, class_id) REFERENCES classes,
    FOREIGN KEY (solution_id, room_id) REFERENCES rooms,
    PRIMARY KEY (solution_id, class_id, room_id)
);