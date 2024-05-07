// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Text,
        part_id -> Text,
    }
}

diesel::table! {
    classes_groups (class_id, group_id) {
        class_id -> Text,
        group_id -> Text,
    }
}

diesel::table! {
    courses (id) {
        id -> Text,
        solution_id -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    parts (id) {
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
    rooms (id) {
        id -> Text,
        solution_id -> Binary,
        capacity -> Integer,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (class_id, rank) {
        class_id -> Text,
        rank -> Integer,
        starting_date -> Timestamp,
    }
}

diesel::table! {
    sessions_rooms (class_id, session_rank, room_id) {
        class_id -> Text,
        session_rank -> Integer,
        room_id -> Text,
    }
}

diesel::table! {
    sessions_teachers (class_id, session_rank, teacher_id) {
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
    students (id) {
        id -> Text,
        solution_id -> Integer,
        name -> Nullable<Text>,
        label -> Nullable<Text>,
    }
}

diesel::table! {
    students_groups (student_id, group_id) {
        student_id -> Text,
        group_id -> Text,
    }
}

diesel::table! {
    teachers (name) {
        name -> Text,
        solution_id -> Integer,
        department -> Nullable<Text>,
    }
}

diesel::joinable!(classes -> parts (part_id));
diesel::joinable!(classes_groups -> classes (class_id));
diesel::joinable!(courses -> solutions (solution_id));
diesel::joinable!(parts -> courses (course_id));
diesel::joinable!(sessions -> classes (class_id));
diesel::joinable!(sessions_rooms -> rooms (room_id));
diesel::joinable!(sessions_teachers -> teachers (teacher_id));
diesel::joinable!(students_groups -> students (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_groups,
    courses,
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
