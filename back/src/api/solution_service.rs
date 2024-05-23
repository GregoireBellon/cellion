use std::error::Error;

use chrono::{DateTime, Datelike, Local, TimeZone, Utc};
use diesel::{self, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection};
use log::warn;
use uuid::Uuid;

use crate::models::{
    db::{
        last_insert_rowid, Class, ClassGroupOwn, ClassRoomOwn, ClassTeacherOwn, Course,
        InsertSolution, Part, Room, Session, SolutionGroupOwn, Student, StudentGroupOwn, Teacher,
    },
    schema::{self},
};

use super::{
    buffer_handler::BufferHandler,
    date_service::{extract_session_date, extract_starting_date},
    xml_types::{
        XmlCalendar, XmlClass, XmlCourse, XmlGroupClasses, XmlGroupStudents, XmlPart, XmlRoom,
        XmlSession, XmlSolutionClass, XmlSolutionClassRooms, XmlSolutionClassTeachers,
        XmlSolutionGroup, XmlStudent, XmlTeacher,
    },
};

pub struct SolutionInserter<'a> {
    conn: &'a mut SqliteConnection,
    solution_id: i32,

    buffer_handler: BufferHandler,

    starting_date: DateTime<Utc>,
    slot_duration: u16,
}

impl<'a> SolutionInserter<'a> {
    pub fn new(
        conn: &'a mut SqliteConnection,
        solution: &InsertSolution,
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(schema::solutions::table)
            .values(solution)
            .execute(conn)?;

        let solution_id = diesel::select(last_insert_rowid()).first::<i32>(conn)?;

        return Ok(SolutionInserter {
            conn: conn,
            solution_id: solution_id,

            buffer_handler: BufferHandler::new(),

            starting_date: Utc
                .with_ymd_and_hms(Local::now().year(), 1, 1, 0, 0, 0)
                .unwrap(),
            slot_duration: 1,
        });
    }

    pub fn solution_id(&self) -> i32 {
        self.solution_id
    }

    pub fn insert_all_into_db(&mut self) -> QueryResult<usize> {
        self.buffer_handler.insert_all_into_db(self.conn)
    }

    pub fn add_calendar(&mut self, calendar: XmlCalendar) -> QueryResult<usize> {
        // Minutes in a day divided by the number of slots in the day
        self.slot_duration = ((60 * 24) / calendar.slots.nr) as u16;

        let extracted_date = extract_starting_date(calendar.year, calendar.starting_week, &Utc);

        match extracted_date {
            Some(extracted) => self.starting_date = extracted,
            None => warn!("The starting date described by calendar is not valid !"),
        }

        diesel::update(schema::solutions::table)
            .filter(schema::solutions::id.eq(self.solution_id))
            .set(schema::solutions::slot_duration.eq(self.slot_duration as i32))
            .execute(self.conn)
    }

    pub fn add_student(&mut self, student: XmlStudent) {
        self.buffer_handler
            .students_to_insert
            .push(student.into_db_entry(self.solution_id));

        self.buffer_handler.on_add_callback(self.conn);
    }

    pub fn add_teacher(&mut self, teacher: XmlTeacher) {
        self.buffer_handler
            .teachers_to_insert
            .push(teacher.into_db_entry(self.solution_id));
        self.buffer_handler.on_add_callback(self.conn);
    }

    pub fn add_room(&mut self, room: XmlRoom) {
        self.buffer_handler
            .rooms_to_insert
            .push(room.into_db_entry(self.solution_id));
        self.buffer_handler.on_add_callback(self.conn);
    }

    pub fn add_solution_group(&mut self, group: XmlSolutionGroup) {
        self.buffer_handler
            .solution_groups_to_insert
            .push(group.into_db_entry(self.solution_id));

        group.classes.map(|classes| {
            self.buffer_handler
                .classes_groups_to_insert
                .extend(classes.into_db_entry(self.solution_id, &group.id));
            self.buffer_handler.on_add_callback(self.conn);
        });
        group.students.map(|students| {
            self.buffer_handler
                .students_groups_to_insert
                .extend(students.into_db_entry(self.solution_id, &group.id));
            self.buffer_handler.on_add_callback(self.conn);
        });
    }

    pub fn add_solution_class(&mut self, class: XmlSolutionClass) {
        class.teachers.map(|teachers| {
            self.buffer_handler
                .classes_teachers_to_insert
                .extend(teachers.into_db_entry(self.solution_id, &class.ref_id));
            self.buffer_handler.on_add_callback(self.conn);
        });

        class.rooms.map(|rooms| {
            self.buffer_handler
                .classes_rooms_to_insert
                .extend(rooms.into_db_entry(self.solution_id, &class.ref_id));
            self.buffer_handler.on_add_callback(self.conn);
        });
    }

    pub fn add_session(&mut self, session: XmlSession) -> Result<(), String> {
        session
            .into_db_entry(self.solution_id, &self.starting_date, self.slot_duration)
            .map(|s| self.buffer_handler.sessions_to_insert.push(s))?;

        session.teachers.map(|teachs| {
            teachs.teachers_id.into_iter().for_each(|t| {
                self.buffer_handler.sessions_teachers_to_insert_queries.push(format!(
                    r#"
                        (SELECT id from sessions WHERE rank = {} AND class_id = "{}" AND solution_id = {}), "{}", "{}"
                    "#, session.rank, session.class.clone(), self.solution_id, t.ref_id, self.solution_id
                ));

                self.buffer_handler.on_add_callback(self.conn);
            })
        });

        session.rooms.map(|s_rooms| {
            s_rooms.rooms_id.into_iter().for_each(|r| {
                self.buffer_handler.sessions_rooms_to_insert_queries.push(format!(
                    r#"
                        (SELECT id from sessions WHERE rank = {} AND class_id = "{}" AND solution_id = {}), "{}", "{}"
                    "#, session.rank, session.class.clone(), self.solution_id, r.ref_id, self.solution_id
                ));

                self.buffer_handler.on_add_callback(self.conn);
            })
        });

        Ok(())
    }

    pub fn add_course(&mut self, course: XmlCourse) -> Result<(), Box<dyn Error>> {
        self.buffer_handler
            .courses_to_insert
            .push(course.into_db_entry(self.solution_id));

        let solution_id = self.solution_id;

        course.parts.into_iter().for_each(|part| {
            self.buffer_handler
                .parts_to_insert
                .push(part.into_db_entry(solution_id, &course.id));

            self.buffer_handler.on_add_callback(self.conn);

            part.classes
                .class
                .into_iter()
                .map(|c| c.into_db_entry(solution_id, &part.id))
                .for_each(|c| {
                    self.buffer_handler.classes_to_insert.push(c);
                    self.buffer_handler.on_add_callback(self.conn);
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
            session_length: self.allowed_slots.session_lenght,
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
    fn into_db_entry(
        &self,
        given_solution_id: i32,
        starting_date: &DateTime<Utc>,
        slot_duration: u16,
    ) -> Result<Session, String> {
        self.extract_date(starting_date, slot_duration)
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
        starting_date: &DateTime<Utc>,
        slot_duration: u16,
    ) -> Option<DateTime<Utc>> {
        if self.starting_slot.is_none() {
            return None;
        }

        let starting_slot = self.starting_slot.as_ref().unwrap();

        return extract_session_date(
            starting_date,
            slot_duration,
            starting_slot.daily_slot,
            starting_slot.week,
            starting_slot.day,
        );
    }
}
