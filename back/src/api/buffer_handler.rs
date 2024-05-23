use std::ops::Deref;

use diesel::{QueryResult, RunQueryDsl, SqliteConnection};
use log::error;

use crate::models::{
    db::{
        Class, ClassGroupOwn, ClassRoomOwn, ClassTeacherOwn, Course, Part, Room, Session,
        SolutionGroupOwn, Student, StudentGroupOwn, Teacher,
    },
    schema,
};

fn use_query<F, R>(base_query: &str, buff: &mut Vec<String>, rows_to_insert: &mut i32, f: F) -> R
where
    F: FnOnce(String) -> R,
{
    let query_nb = buff.len();

    let header = format!("INSERT OR IGNORE INTO {} VALUES ", base_query);

    let capacity_needed = base_query.len()
        + buff.deref().iter().map(String::len).sum::<usize>()
        + header.len()
        + (2 * query_nb);

    let mut query = String::with_capacity(capacity_needed);

    query.push_str(header.as_str());

    for i in 0..query_nb {
        let buff_element = buff.pop();

        match buff_element {
            Some(elem) => {
                *rows_to_insert -= 1;
                query.push('(');
                query.push_str(elem.as_str());
                query.push(')');
                if i != query_nb - 1 {
                    query.push(',');
                }
            }
            None => break,
        }
    }

    let ret = f(query);

    buff.clear();

    return ret;
}

fn use_buffer<T, F, R>(buff: &mut Vec<T>, rows_to_insert: &mut i32, f: F) -> R
where
    F: FnOnce(&Vec<T>) -> R,
{
    let ret = f(&buff);

    *rows_to_insert -= buff.len() as i32;
    buff.clear();

    return ret;
}

pub struct BufferHandler {
    pub rows_to_insert: i32,

    pub rooms_to_insert: Vec<Room>,
    pub teachers_to_insert: Vec<Teacher>,
    pub classes_to_insert: Vec<Class>,
    pub courses_to_insert: Vec<Course>,
    pub parts_to_insert: Vec<Part>,
    pub students_to_insert: Vec<Student>,
    pub solution_groups_to_insert: Vec<SolutionGroupOwn>,
    pub students_groups_to_insert: Vec<StudentGroupOwn>,
    pub sessions_to_insert: Vec<Session>,
    pub sessions_teachers_to_insert_queries: Vec<String>,
    pub sessions_rooms_to_insert_queries: Vec<String>,
    pub classes_groups_to_insert: Vec<ClassGroupOwn>,
    pub classes_teachers_to_insert: Vec<ClassTeacherOwn>,
    pub classes_rooms_to_insert: Vec<ClassRoomOwn>,
}

impl BufferHandler {
    pub fn new() -> Self {
        BufferHandler {
            rows_to_insert: 0,
            rooms_to_insert: Vec::new(),
            teachers_to_insert: Vec::new(),
            classes_to_insert: Vec::new(),
            courses_to_insert: Vec::new(),
            parts_to_insert: Vec::new(),
            students_to_insert: Vec::new(),
            solution_groups_to_insert: Vec::new(),
            students_groups_to_insert: Vec::new(),
            sessions_to_insert: Vec::new(),
            sessions_teachers_to_insert_queries: Vec::new(),
            sessions_rooms_to_insert_queries: Vec::new(),
            classes_groups_to_insert: Vec::new(),
            classes_teachers_to_insert: Vec::new(),
            classes_rooms_to_insert: Vec::new(),
        }
    }

    pub fn on_add_callback(&mut self, conn: &mut SqliteConnection) {
        self.rows_to_insert += 1;

        if self.rows_to_insert > 20000 {
            if let Err(e) = self.insert_all_into_db(conn) {
                error!("Error while dumping the buffer : {e}");
            }
        }
    }

    // I would like to refactor this, but Diesel typing makes it really really hard....
    // Storing the pair (vector, table) into a vector and iterating on it would be amazing
    pub fn insert_all_into_db(&mut self, conn: &mut SqliteConnection) -> QueryResult<usize> {
        let mut nb_inserted: usize = 0;

        nb_inserted += use_buffer(&mut self.rooms_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::rooms::table)
                .values(b)
                .execute(conn)
        })?;

        nb_inserted += use_buffer(
            &mut self.teachers_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::teachers::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(&mut self.classes_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::classes::table)
                .values(b)
                .execute(conn)
        })?;

        nb_inserted += use_buffer(&mut self.courses_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::courses::table)
                .values(b)
                .execute(conn)
        })?;

        nb_inserted += use_buffer(&mut self.parts_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_into(schema::parts::table)
                .values(b)
                .execute(conn)
        })?;

        nb_inserted += use_buffer(
            &mut self.students_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::students::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.solution_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::groups::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.students_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::students_groups::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.sessions_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::sessions::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_query(
            "sessions_teachers(session_id, teacher_id, solution_id)",
            &mut self.sessions_teachers_to_insert_queries,
            &mut self.rows_to_insert,
            |q| diesel::sql_query(q).execute(conn),
        )?;

        nb_inserted += use_query(
            "sessions_rooms(session_id, room_id, solution_id)",
            &mut self.sessions_rooms_to_insert_queries,
            &mut self.rows_to_insert,
            |q| diesel::sql_query(q).execute(conn),
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_groups::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_teachers_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_teachers::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_rooms_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_rooms::table)
                    .values(b)
                    .execute(conn)
            },
        )?;

        return Ok(nb_inserted);
    }
}
