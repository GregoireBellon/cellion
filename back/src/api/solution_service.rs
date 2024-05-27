use diesel::{self, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection};
use uuid::Uuid;

use crate::models::{
    db::{
        last_insert_rowid, Class, ClassGroupOwn, ClassRoomOwn, ClassTeacherOwn, Course,
        InsertSolution, Part, Room, Session, SolutionGroupOwn, Student, StudentGroupOwn, Teacher,
    },
    schema::{self},
};

use crate::xml_parsing::types::{
    XmlCalendar, XmlClass, XmlCourse, XmlGroupClasses, XmlGroupStudents, XmlPart, XmlRoom,
    XmlSession, XmlSolutionClass, XmlSolutionClassRooms, XmlSolutionClassTeachers,
    XmlSolutionGroup, XmlStudent, XmlTeacher,
};

use super::{buffer_handler::BufferHandler, calendar_handler::CalendarHandler};

pub struct SolutionInserter<'a> {
    conn: &'a mut SqliteConnection,
    solution_id: i32,

    buffer_handler: BufferHandler,
    calendar_data_handler: CalendarHandler,
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
            calendar_data_handler: CalendarHandler::new(),
        });
    }

    pub fn solution_id(&self) -> i32 {
        self.solution_id
    }

    pub fn insert_all_into_db(&mut self) -> QueryResult<usize> {
        self.buffer_handler.insert_all_into_db(self.conn)
    }

    pub fn add_calendar(&mut self, xml_calendar: XmlCalendar) -> QueryResult<usize> {
        self.calendar_data_handler
            .register_xml_calendar(&xml_calendar);

        diesel::update(schema::solutions::table)
            .filter(schema::solutions::id.eq(self.solution_id))
            .set((
                schema::solutions::slot_duration
                    .eq(self.calendar_data_handler.slot_duration as i32),
                schema::solutions::calendar_start.eq(self.calendar_data_handler.starting_date),
            ))
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
            .into_db_entry(self.solution_id, &self.calendar_data_handler)
            .map(|s| self.buffer_handler.sessions_to_insert.push(s))?;

        session.teachers.map(|teachs| {
            teachs.teachers_id.into_iter().for_each(|t| {
                let query = format!(
                    r#"
                        (SELECT id from sessions WHERE rank = {} AND class_id = "{}" AND solution_id = {}), "{}", "{}"
                    "#, session.rank, session.class.clone(), self.solution_id, t.ref_id, self.solution_id
                );
                // debug!("query for session_teacher: {}", query);
                self.buffer_handler.sessions_teachers_to_insert_queries.push(query);
                self.buffer_handler.on_add_callback(self.conn);
            })
        });

        session.rooms.map(|s_rooms| {
            s_rooms.rooms_id.into_iter().for_each(|r| {
                let query = format!(
                    r#"
                        (SELECT id from sessions WHERE rank = {} AND class_id = "{}" AND solution_id = {}), "{}", "{}"
                    "#, session.rank, session.class.clone(), self.solution_id, r.ref_id, self.solution_id
                );

                self.buffer_handler.sessions_rooms_to_insert_queries.push(query);

                self.buffer_handler.on_add_callback(self.conn);
            })
        });

        Ok(())
    }

    pub fn add_course(&mut self, course: XmlCourse) {
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
        calendar_handler: &CalendarHandler,
    ) -> Result<Session, String> {
        if self.starting_slot.is_none() {
            return Err(String::from("This session doesn't have any date."));
        }

        let starting_slot = self.starting_slot.as_ref().unwrap();

        calendar_handler
            .extract_session_date(
                starting_slot.daily_slot,
                starting_slot.week,
                starting_slot.day,
            )
            .map(|date| Session {
                solution_id: given_solution_id,
                uuid: Uuid::new_v4().to_string(),
                class_id: self.class.to_string(),
                rank: self.rank,
                starting_date: date,
            })
            .ok_or(String::from("The date is invalid"))
    }
}
