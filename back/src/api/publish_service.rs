use std::error::Error;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use diesel::{self, QueryResult, RunQueryDsl, SqliteConnection};
use log::error;
use uuid::Uuid;

use crate::{
    models::db::{
        last_insert_rowid, Class, ClassGroupOwn, ClassRoomOwn, ClassTeacherOwn, Course,
        InsertSolution, Part, Room, Session, SessionRoomOwn, SessionTeacherOwn, SolutionGroupOwn,
        Student, StudentGroupOwn, Teacher,
    },
    schema,
};

use super::xml_types::{
    XmlClass, XmlCourse, XmlGroupClasses, XmlGroupStudents, XmlPart, XmlRoom, XmlSession,
    XmlSolutionClass, XmlSolutionClassRooms, XmlSolutionClassTeachers, XmlSolutionGroup,
    XmlStudent, XmlTeacher,
};

fn use_buffer<T, F, R>(buff: &mut Vec<T>, rows_to_insert: &mut i32, f: F) -> R
where
    F: FnOnce(&Vec<T>) -> R,
{
    let ret = f(&buff);

    *rows_to_insert -= buff.len() as i32;
    buff.clear();

    return ret;
}

pub struct SolutionInserter<'a> {
    conn: &'a mut SqliteConnection,
    solution_id: i32,

    rows_to_insert: i32,

    rooms_to_insert: Vec<Room>,
    teachers_to_insert: Vec<Teacher>,
    classes_to_insert: Vec<Class>,
    parts_to_insert: Vec<Part>,
    courses_to_insert: Vec<Course>,
    students_to_insert: Vec<Student>,
    solution_groups_to_insert: Vec<SolutionGroupOwn>,
    students_groups_to_insert: Vec<StudentGroupOwn>,
    sessions_to_insert: Vec<Session>,
    sessions_teachers_to_insert: Vec<SessionTeacherOwn>,
    sessions_rooms_to_insert: Vec<SessionRoomOwn>,
    classes_groups_to_insert: Vec<ClassGroupOwn>,
    classes_teachers_to_insert: Vec<ClassTeacherOwn>,
    classes_rooms_to_insert: Vec<ClassRoomOwn>,
}

impl<'a> SolutionInserter<'a> {
    pub fn new(
        conn: &'a mut SqliteConnection,
        solution: &InsertSolution,
    ) -> Result<SolutionInserter<'a>, diesel::result::Error> {
        diesel::insert_into(schema::solutions::table)
            .values(solution)
            .execute(conn)?;

        let solution_id = diesel::select(last_insert_rowid()).first::<i32>(conn)?;

        return Ok(SolutionInserter {
            conn: conn,
            solution_id: solution_id,
            rows_to_insert: 0,
            rooms_to_insert: Vec::new(),
            teachers_to_insert: Vec::new(),
            classes_to_insert: Vec::new(),
            parts_to_insert: Vec::new(),
            courses_to_insert: Vec::new(),
            students_to_insert: Vec::new(),
            solution_groups_to_insert: Vec::new(),
            students_groups_to_insert: Vec::new(),
            sessions_to_insert: Vec::new(),
            sessions_teachers_to_insert: Vec::new(),
            sessions_rooms_to_insert: Vec::new(),
            classes_groups_to_insert: Vec::new(),
            classes_teachers_to_insert: Vec::new(),
            classes_rooms_to_insert: Vec::new(),
        });
    }

    pub fn solution_id(&self) -> i32 {
        self.solution_id
    }

    fn on_add_callback(&mut self) {
        self.rows_to_insert += 1;

        if self.rows_to_insert > 20000 {
            if let Err(e) = self.insert_all_into_db() {
                error!("Error while dumping the buffer : {e}");
            }
        }
    }

    // I would like to refactor this, but Diesel typing makes it really really hard....
    // Storing the pair (vector, table) into a vector and iterating on it would be amazing
    pub fn insert_all_into_db(&mut self) -> QueryResult<usize> {
        let mut nb_inserted: usize = 0;

        nb_inserted += use_buffer(&mut self.rooms_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::rooms::table)
                .values(b)
                .execute(self.conn)
        })?;

        nb_inserted += use_buffer(
            &mut self.teachers_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::teachers::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(&mut self.classes_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::classes::table)
                .values(b)
                .execute(self.conn)
        })?;

        nb_inserted += use_buffer(&mut self.parts_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::parts::table)
                .values(b)
                .execute(self.conn)
        })?;

        nb_inserted += use_buffer(&mut self.courses_to_insert, &mut self.rows_to_insert, |b| {
            diesel::insert_or_ignore_into(schema::courses::table)
                .values(b)
                .execute(self.conn)
        })?;

        nb_inserted += use_buffer(
            &mut self.students_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::students::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.solution_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::groups::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.students_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::students_groups::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.sessions_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::sessions::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.sessions_teachers_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::sessions_teachers::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.sessions_rooms_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::sessions_rooms::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_groups_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_groups::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_teachers_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_teachers::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        nb_inserted += use_buffer(
            &mut self.classes_rooms_to_insert,
            &mut self.rows_to_insert,
            |b| {
                diesel::insert_or_ignore_into(schema::classes_rooms::table)
                    .values(b)
                    .execute(self.conn)
            },
        )?;

        return Ok(nb_inserted);
    }

    pub fn add_student(&mut self, student: XmlStudent) {
        self.students_to_insert
            .push(student.into_db_entry(self.solution_id));

        self.on_add_callback();
    }

    pub fn add_teacher(&mut self, teacher: XmlTeacher) {
        self.teachers_to_insert
            .push(teacher.into_db_entry(self.solution_id));
        self.on_add_callback();
    }

    pub fn add_room(&mut self, room: XmlRoom) {
        self.rooms_to_insert
            .push(room.into_db_entry(self.solution_id));
        self.on_add_callback();
    }

    pub fn add_solution_group(&mut self, group: XmlSolutionGroup) {
        self.solution_groups_to_insert
            .push(group.into_db_entry(self.solution_id));

        group.classes.map(|classes| {
            self.classes_groups_to_insert
                .extend(classes.into_db_entry(self.solution_id, &group.id));
            self.on_add_callback();
        });
        group.students.map(|students| {
            self.students_groups_to_insert
                .extend(students.into_db_entry(self.solution_id, &group.id));
            self.on_add_callback();
        });
    }

    pub fn add_solution_class(&mut self, class: XmlSolutionClass) {
        class.teachers.map(|teachers| {
            self.classes_teachers_to_insert
                .extend(teachers.into_db_entry(self.solution_id, &class.ref_id));
            self.on_add_callback();
        });

        class.rooms.map(|rooms| {
            self.classes_rooms_to_insert
                .extend(rooms.into_db_entry(self.solution_id, &class.ref_id));
            self.on_add_callback();
        });
    }

    pub fn add_session(&mut self, session: XmlSession) -> Result<(), String> {
        session
            .into_db_entry(self.solution_id)
            .map(|s| self.sessions_to_insert.push(s))?;

        session.teachers.map(|teachs| {
            teachs.teachers_id.into_iter().for_each(|t| {
                self.sessions_teachers_to_insert.push(SessionTeacherOwn {
                    class_id: session.class.clone(),
                    teacher_id: t.ref_id,
                    session_rank: session.rank,
                    solution_id: self.solution_id,
                });

                self.on_add_callback();
            })
        });

        session.rooms.map(|s_rooms| {
            s_rooms.rooms_id.into_iter().for_each(|r| {
                self.sessions_rooms_to_insert.push(SessionRoomOwn {
                    solution_id: self.solution_id,
                    class_id: session.class.clone(),
                    session_rank: session.rank,
                    room_id: r.ref_id,
                });

                self.on_add_callback();
            })
        });

        Ok(())
    }

    pub fn add_course(&mut self, course: XmlCourse) -> Result<(), Box<dyn Error>> {
        self.courses_to_insert
            .push(course.into_db_entry(self.solution_id));

        let solution_id = self.solution_id;

        course.parts.into_iter().for_each(|part| {
            self.parts_to_insert
                .push(part.into_db_entry(solution_id, &course.id));

            self.on_add_callback();

            part.classes
                .class
                .into_iter()
                .map(|c| c.into_db_entry(solution_id, &part.id))
                .for_each(|c| {
                    self.classes_to_insert.push(c);
                    self.on_add_callback();
                });
        });

        return Ok(());
    }
}

impl XmlClass {
    fn into_db_entry(self, given_solution_id: i32, given_part_id: &str) -> Class {
        Class {
            solution_id: given_solution_id,
            id: self.id,
            part_id: given_part_id.to_string(),
        }
    }
}

impl XmlTeacher {
    fn into_db_entry(self, given_solution_id: i32) -> Teacher {
        Teacher {
            solution_id: given_solution_id,
            name: self.id,
            department: self.label,
        }
    }
}

impl XmlRoom {
    fn into_db_entry(self, given_solution_id: i32) -> Room {
        Room {
            solution_id: given_solution_id,
            id: self.id,
            capacity: self.capacity,
            name: self.label,
        }
    }
}

impl XmlCourse {
    fn into_db_entry(&self, given_solution_id: i32) -> Course {
        Course {
            solution_id: given_solution_id,
            id: self.id.clone(),
            name: self.label.clone(),
        }
    }
}

impl XmlPart {
    fn into_db_entry(&self, given_solution_id: i32, given_course_id: &str) -> Part {
        Part {
            solution_id: given_solution_id,
            id: self.id.clone(),
            course_id: given_course_id.to_string(),
            session_teachers: None,
            session_rooms: None,
            label: self.label.clone(),
            max_head_count: self.classes.max_head_count,
            nr_session: None,
        }
    }
}

impl XmlStudent {
    fn into_db_entry(self, given_solution_id: i32) -> Student {
        Student {
            solution_id: given_solution_id,
            id: self.id,
            label: self.label,
        }
    }
}

impl XmlGroupClasses {
    fn into_db_entry(
        self,
        given_solution_id: i32,
        given_group_id: &str,
    ) -> impl Iterator<Item = ClassGroupOwn> + '_ {
        self.classes.into_iter().map(move |c| ClassGroupOwn {
            solution_id: given_solution_id,
            group_id: given_group_id.to_string(),
            class_id: c.ref_id,
        })
    }
}

impl XmlGroupStudents {
    fn into_db_entry(
        self,
        given_solution_id: i32,
        given_group_id: &str,
    ) -> impl Iterator<Item = StudentGroupOwn> + '_ {
        self.students.into_iter().map(move |s| StudentGroupOwn {
            solution_id: given_solution_id,
            group_id: given_group_id.to_string(),
            student_id: s.ref_id,
        })
    }
}

impl XmlSolutionGroup {
    fn into_db_entry(&self, given_solution_id: i32) -> SolutionGroupOwn {
        SolutionGroupOwn {
            id: self.id.to_owned(),
            solution_id: given_solution_id,
        }
    }
}

impl XmlSolutionClassRooms {
    fn into_db_entry(
        self,
        given_solution_id: i32,
        given_class_id: &str,
    ) -> impl Iterator<Item = ClassRoomOwn> + '_ {
        self.rooms_id.into_iter().map(move |t| ClassRoomOwn {
            class_id: given_class_id.to_string(),
            solution_id: given_solution_id,
            room_id: t.ref_id,
        })
    }
}
impl XmlSolutionClassTeachers {
    fn into_db_entry(
        self,
        given_solution_id: i32,
        given_class_id: &str,
    ) -> impl Iterator<Item = ClassTeacherOwn> + '_ {
        self.teachers_id.into_iter().map(move |t| ClassTeacherOwn {
            class_id: given_class_id.to_string(),
            solution_id: given_solution_id,
            teacher_id: t.ref_id,
        })
    }
}

impl XmlSession {
    fn into_db_entry(&self, given_solution_id: i32) -> Result<Session, String> {
        self.extract_date(
            &NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1).unwrap(),
            &Local,
        )
        .map(|date| Session {
            solution_id: given_solution_id,
            uuid: Uuid::new_v4().to_string(),
            class_id: self.class.to_string(),
            rank: self.rank,
            starting_date: date.naive_utc(),
        })
        .ok_or(String::from("La date est invalide"))
    }

    fn extract_date(
        &self,
        starting_date: &NaiveDate,
        timezone: &impl TimeZone,
    ) -> Option<DateTime<Utc>> {
        let mut extracted_date: NaiveDate = starting_date.clone();

        if self.starting_slot.is_none() {
            return None;
        }

        let starting_slot = &self.starting_slot.as_ref().unwrap();

        extracted_date = starting_date
            .checked_add_signed(chrono::Duration::weeks(starting_slot.week as i64))
            .unwrap_or(extracted_date);

        extracted_date = starting_date
            .checked_add_signed(chrono::Duration::days(starting_slot.day as i64))
            .unwrap_or(extracted_date);

        let extracted_time =
            NaiveTime::from_num_seconds_from_midnight_opt(60 * starting_slot.daily_slot, 0)?;

        let extracted_date_time = timezone
            .from_local_datetime(&NaiveDateTime::new(extracted_date, extracted_time))
            .single()
            .unwrap();

        return Some(extracted_date_time.with_timezone(&Utc));
    }
}
