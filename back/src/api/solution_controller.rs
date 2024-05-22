use std::{
    error::Error,
    io::BufRead,
    path::Path,
    str::{self, FromStr},
};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error as actix_error, post, web, Error as ActixError, HttpResponse, Responder};
use chrono::Utc;
use diesel::ExpressionMethods;
use log::{debug, error};
use quick_xml::{
    de::from_str,
    events::{BytesStart, Event},
    Reader, Writer,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::{
        solution_service::SolutionInserter,
        xml_types::{XmlRoom, XmlTeacher},
    },
    models::schema,
    DbPool,
};

use super::xml_types::{
    XmlCalendar, XmlCourse, XmlSession, XmlSolutionClass, XmlSolutionGroup, XmlStudent,
};

#[derive(MultipartForm)]
struct SolutionUpload {
    #[multipart(rename = "solution")]
    file: TempFile,
}

#[derive(Serialize)]
struct UploadResult {
    pub id: i32,
    pub row_inserted: usize,
}

#[post("")]
pub async fn post_route(
    payload: MultipartForm<SolutionUpload>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    match payload.file.content_type.as_ref() {
        Some(ct) => {
            if ct.subtype() != mime::XML {
                return Result::Err(actix_error::ErrorBadRequest(format!(
                    "The file does not seem to be XML (it is {ct})"
                )));
            }
        }
        None => return Result::Err(actix_error::ErrorBadRequest("No content")),
    };

    web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let mut solution_inserter = SolutionInserter::new(
            &mut conn,
            &(
                schema::solutions::filename.eq(payload
                    .file
                    .file_name
                    .as_deref()
                    .unwrap_or("UNKNOWN")),
                schema::solutions::created_at.eq(&Utc::now().naive_utc()),
            ),
        )
        .map_err(|e| e.to_string())?;

        debug!("Solution inserted");

        extract_file(payload.file.file.path(), &mut solution_inserter)
            .map_err(|e| e.to_string())?;

        debug!("File extracted ");

        return solution_inserter
            .insert_all_into_db()
            .map(|inserted| UploadResult {
                id: solution_inserter.solution_id(),
                row_inserted: inserted,
            })
            .map_err(|e| e.to_string());
    })
    .await?
    .map(|result| HttpResponse::Ok().json(result))
    .map_err(|e| actix_error::ErrorBadRequest(format!("Error while processing the file : {e}")))
}

fn extract_file(file: &Path, solution_inserter: &mut SolutionInserter) -> Result<(), String> {
    let mut reader = Reader::from_file(file).map_err(|e| e.to_string())?;

    let mut buffer = Vec::new();
    let mut serialization_buffer = Vec::new();
    let mut junk_buffer = Vec::new();

    let mut route: Vec<String> = Vec::new();

    loop {
        buffer.clear();
        let event_result = reader.read_event_into(&mut buffer);

        match event_result {
            Err(e) => return Err(e.to_string()),
            Ok(event) => match event.as_ref() {
                Event::Eof => break,

                Event::Start(bs) => {
                    route.push(
                        String::from_str(str::from_utf8(bs.name().into_inner()).unwrap()).unwrap(),
                    );
                    handle_xml_event(
                        event,
                        &mut reader,
                        &mut junk_buffer,
                        &mut serialization_buffer,
                        &mut route,
                        solution_inserter,
                    );
                }
                Event::End(_) => {
                    handle_xml_event(
                        event,
                        &mut reader,
                        &mut junk_buffer,
                        &mut serialization_buffer,
                        &mut route,
                        solution_inserter,
                    );
                    route.pop();
                }

                _ => handle_xml_event(
                    event,
                    &mut reader,
                    &mut junk_buffer,
                    &mut serialization_buffer,
                    &mut route,
                    solution_inserter,
                ),
            },
        };
    }

    return Result::Ok(());
}

fn handle_xml_event<R: BufRead>(
    event: Event,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
    route: &mut Vec<String>,
    solution_inserter: &mut SolutionInserter,
) {
    match event {
        Event::Empty(bytes_start) => {
            if route.len() == 2
                && route.get(1).unwrap() == "rooms"
                && bytes_start.name().into_inner() == b"room"
            {
                handle_room_declaration(solution_inserter, bytes_start, serialization_buffer);
            } else if route.len() == 2
                && route.get(1).unwrap() == "teachers"
                && bytes_start.name().into_inner() == b"teacher"
            {
                handle_teachers_declaration(solution_inserter, bytes_start, serialization_buffer)
            }
        }

        Event::Start(bytes_start) => {
            if route.len() == 3
                && route.get(1).unwrap() == "students"
                && bytes_start.name().into_inner() == b"student"
            {
                // debug!("In student");
                handle_students_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                );

                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 4
                && route.get(1).unwrap() == "solution"
                && route.get(2).unwrap() == "sessions"
                && bytes_start.name().into_inner() == b"session"
            {
                // debug!("In session");
                handle_solution_session_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                );
                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 3
                && route.get(1).unwrap() == "courses"
                && bytes_start.name().into_inner() == b"course"
            {
                // debug!("In course");
                handle_course_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                )
                .ok();

                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 4
                && route.get(1).unwrap() == "solution"
                && route.get(2).unwrap() == "groups"
                && bytes_start.name().into_inner() == b"group"
            {
                // debug!("In solution group");
                handle_solution_group_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                );
                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 4
                && route.get(1).unwrap() == "solution"
                && route.get(2).unwrap() == "classes"
                && bytes_start.name().into_inner() == b"class"
            {
                // debug!("In solution class");
                handle_solution_class_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                );
                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 2
                && route.get(0).unwrap() == "timetabling"
                && bytes_start.name().into_inner() == b"calendar"
            {
                debug!("In calendar");
                handle_calendar_declaration(
                    solution_inserter,
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                )
                .ok();

                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            }
        }

        _ => (),
    }
}

fn handle_solution_session_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSession, R>(reader, &node, buffer, serialization_buffer) {
        Ok(session) => {
            solution_inserter.add_session(session).ok();
        }
        Err(e) => error!("Error while deserializing a session from solution: {}", e),
    };
}

fn handle_solution_class_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSolutionClass, R>(reader, &node, buffer, serialization_buffer) {
        Ok(class) => {
            solution_inserter.add_solution_class(class);
        }
        Err(e) => {
            error!("Error while deserializing class from solution: {}", e);
        }
    }
}
fn handle_solution_group_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSolutionGroup, R>(reader, &node, buffer, serialization_buffer) {
        Ok(group) => {
            solution_inserter.add_solution_group(group);
        }
        Err(e) => {
            error!("Error while deserializing group from solution: {}", e);
        }
    }
}
fn handle_course_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match deserialize_element::<XmlCourse, R>(reader, &node, buffer, serialization_buffer) {
        Ok(course) => solution_inserter.add_course(course),
        Err(e) => {
            error!("Error while deserializing course: {}", e);
            return Err(e);
        }
    }
}

fn handle_calendar_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match deserialize_element::<XmlCalendar, R>(reader, &node, buffer, serialization_buffer) {
        Ok(calendar) => match solution_inserter.add_calendar(calendar) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error while getting the calendar : {}", e);
                Err(Box::from(e))
            }
        },
        Err(e) => {
            error!("Error while deserializing calendar : {}", e);
            return Err(e);
        }
    }
}

fn handle_room_declaration(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_empty_element::<XmlRoom>(node, serialization_buffer) {
        Ok(room) => solution_inserter.add_room(room),
        Err(e) => {
            error!("Error while deserializing room : {}", e);
        }
    }
}

fn handle_students_declaration<R: BufRead>(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlStudent, R>(reader, &node, buffer, serialization_buffer) {
        Ok(student) => solution_inserter.add_student(student),
        Err(e) => {
            error!("Error while deserializing teacher: {}", e);
        }
    }
}
fn handle_teachers_declaration(
    solution_inserter: &mut SolutionInserter,
    node: BytesStart,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_empty_element::<XmlTeacher>(node, serialization_buffer) {
        Ok(teacher) => solution_inserter.add_teacher(teacher),
        Err(e) => error!("Error while deserializing teacher: {}", e),
    }
}

fn deserialize_empty_element<T: DeserializeOwned>(
    element: BytesStart,
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    output_buffer.clear();

    let mut w = Writer::new(&mut *output_buffer);
    w.write_event(Event::Empty(element))
        .map_err(Box::<quick_xml::Error>::from)?;

    return deserialize_from_buffer(output_buffer);
}

/// Returns an Utf8Error, or a DeError
fn deserialize_from_buffer<T: DeserializeOwned>(
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    let str = std::str::from_utf8(&output_buffer).map_err(|e| Box::new(e))?;

    return from_str::<T>(str).map_err(|e| Box::from(e));
}

/// From https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
fn deserialize_element<T: DeserializeOwned, R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    let mut depth: u8 = 0;
    output_buffer.clear();
    let mut w = Writer::new(&mut *output_buffer);

    let tag_name = start_tag.name();

    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return deserialize_from_buffer(output_buffer);
                }
                depth -= 1;
            }
            Event::Eof => return Err(Box::from("Bad formatting")),
            _ => {}
        }
    }
}
