// @generated automatically by Diesel CLI.

diesel::table! {
    classes (solution_id, id) {
        solution_id -> Integer,
        id -> Text,
        part_id -> Text,
    }
}

diesel::table! {
    classes_groups (solution_id, class_id, group_id) {
        solution_id -> Integer,
        class_id -> Text,
        group_id -> Text,
    }
}

diesel::table! {
    classes_rooms (solution_id, class_id, room_id) {
        solution_id -> Integer,
        class_id -> Text,
        room_id -> Text,
    }
}

diesel::table! {
    classes_teachers (solution_id, class_id, teacher_id) {
        solution_id -> Integer,
        class_id -> Text,
        teacher_id -> Text,
    }
}

diesel::table! {
    courses (solution_id, id) {
        solution_id -> Nullable<Integer>,
        id -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    groups (solution_id, id) {
        solution_id -> Integer,
        id -> Text,
    }
}

diesel::table! {
    parts (solution_id, id) {
        solution_id -> Integer,
        id -> Text,
        course_id -> Text,
        session_teachers -> Nullable<Integer>,
        session_rooms -> Nullable<Text>,
        session_length -> Nullable<Integer>,
        label -> Nullable<Text>,
        max_head_count -> Nullable<Integer>,
        nr_session -> Nullable<Integer>,
    }
}

diesel::table! {
    rooms (solution_id, id) {
        solution_id -> Integer,
        id -> Text,
        capacity -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (solution_id, class_id, rank) {
        solution_id -> Integer,
        uuid -> Text,
        class_id -> Text,
        rank -> Integer,
        starting_date -> Timestamp,
    }
}

diesel::table! {
    sessions_rooms (solution_id, class_id, session_rank, room_id) {
        solution_id -> Integer,
        class_id -> Text,
        session_rank -> Integer,
        room_id -> Text,
    }
}

diesel::table! {
    sessions_teachers (solution_id, class_id, session_rank, teacher_id) {
        solution_id -> Integer,
        class_id -> Text,
        session_rank -> Integer,
        teacher_id -> Text,
    }
}

diesel::table! {
    solutions (id) {
        id -> Integer,
        filename -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    students (solution_id, id) {
        solution_id -> Integer,
        id -> Text,
        label -> Nullable<Text>,
    }
}

diesel::table! {
    students_groups (solution_id, student_id, group_id) {
        solution_id -> Integer,
        student_id -> Text,
        group_id -> Text,
    }
}

diesel::table! {
    teachers (solution_id, name) {
        solution_id -> Integer,
        name -> Text,
        department -> Nullable<Text>,
    }
}

diesel::joinable!(classes -> solutions (solution_id));
diesel::joinable!(classes_groups -> solutions (solution_id));
diesel::joinable!(classes_rooms -> solutions (solution_id));
diesel::joinable!(classes_teachers -> solutions (solution_id));
diesel::joinable!(courses -> solutions (solution_id));
diesel::joinable!(groups -> solutions (solution_id));
diesel::joinable!(parts -> solutions (solution_id));
diesel::joinable!(rooms -> solutions (solution_id));
diesel::joinable!(sessions -> solutions (solution_id));
diesel::joinable!(sessions_teachers -> solutions (solution_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_groups,
    classes_rooms,
    classes_teachers,
    courses,
    groups,
    parts,
    rooms,
    sessions,
    sessions_rooms,
    sessions_teachers,
    solutions,
    students,
    students_groups,
    teachers,
);
