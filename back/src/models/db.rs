use chrono::NaiveDateTime;
use diesel::{dsl::Eq, prelude::*};
use serde::Serialize;

use crate::models::schema;

pub type InsertSolution<'a> = (
    Eq<schema::solutions::filename, &'a str>,
    Eq<schema::solutions::created_at, &'a NaiveDateTime>,
);

#[derive(Queryable, Serialize, Selectable, Debug)]
#[diesel(table_name = schema::solutions)]
pub struct Solution {
    pub id: i32,
    pub filename: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::teachers)]
#[diesel(belongs_to(Solution))]
pub struct Teacher {
    pub name: String,
    pub solution_id: i32,
    pub department: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::sessions_rooms)]
pub struct SessionRoom {
    pub session_id: i32,
    pub solution_id: i32,
    pub room_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::rooms)]
#[diesel(belongs_to(Solution))]
pub struct Room {
    pub id: String,
    pub solution_id: i32,
    pub capacity: i32,
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::courses)]
#[diesel(belongs_to(Solution))]
pub struct Course {
    pub id: String,
    pub solution_id: i32,
    pub name: Option<String>,
}

#[derive(Hash, Eq, PartialEq, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = schema::parts)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(Solution))]
pub struct Part {
    pub solution_id: i32,
    pub id: String,
    pub course_id: String,

    pub session_length: i32,
    pub session_teachers: Option<i32>,
    pub session_rooms: Option<String>,
    pub label: Option<String>,
    pub max_head_count: Option<i32>,
    pub nr_session: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Part))]
#[diesel(belongs_to(Solution))]
#[diesel(table_name = schema::classes)]
pub struct Class {
    pub solution_id: i32,
    pub id: String,
    pub part_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Part))]
#[diesel(belongs_to(Solution))]
#[diesel(table_name = schema::students )]
pub struct Student {
    pub solution_id: i32,
    pub id: String,
    pub label: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::students_groups)]
pub struct StudentGroupOwn {
    pub solution_id: i32,
    pub student_id: String,
    pub group_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::sessions)]
pub struct Session {
    pub solution_id: i32,
    pub uuid: String,
    pub class_id: String,
    pub rank: i32,
    pub starting_date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::sessions_rooms)]
#[diesel(belongs_to(schema::sessions))]
pub struct SessionRoomOwn {
    pub session_id: i32,
    pub room_id: String,
    pub solution_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::sessions_teachers)]
pub struct SessionTeacherOwn {
    pub session_id: i32,
    pub solution_id: i32,
    pub teacher_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::groups)]

pub struct SolutionGroupOwn {
    pub solution_id: i32,
    pub id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::classes_groups)]
pub struct ClassGroupOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub group_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::classes_teachers)]
pub struct ClassTeacherOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub teacher_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = schema::classes_rooms)]
pub struct ClassRoomOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub room_id: String,
}

sql_function! {
    fn last_insert_rowid() -> Integer
}
