use std::{
    error::Error,
    io::BufRead,
    path::Path,
    str::{self, from_utf8, FromStr},
};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error as actix_error, post, Error as ActixError, HttpResponse, Responder};
use log::{debug, error};
use quick_xml::{
    de::from_str,
    events::{BytesStart, Event},
    Reader, Writer,
};
use serde::de::DeserializeOwned;

use crate::api::xml_types::{XmlRoom, XmlTeacher};

use super::xml_types::{XmlCourse, XmlSession, XmlSolutionClass, XmlSolutionGroup};

#[derive(MultipartForm)]
struct SolutionUpload {
    #[multipart(rename = "solution")]
    file: TempFile,
}
#[post("/solution")]
pub async fn post_route(
    payload: MultipartForm<SolutionUpload>,
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

    match extract_file(payload.file.file.path()) {
        Ok(()) => Ok(HttpResponse::Ok().body("The solution has been processed")),
        Err(str) => Result::Err(actix_error::ErrorBadRequest(format!(
            "Error while processing the file : {str}"
        ))),
    }
}

fn extract_file(file: &Path) -> Result<(), String> {
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
                    );
                }
                Event::End(_) => {
                    handle_xml_event(
                        event,
                        &mut reader,
                        &mut junk_buffer,
                        &mut serialization_buffer,
                        &mut route,
                    );
                    route.pop();
                }

                _ => handle_xml_event(
                    event,
                    &mut reader,
                    &mut junk_buffer,
                    &mut serialization_buffer,
                    &mut route,
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
) {
    match event {
        Event::Empty(bytes_start) => {
            if route.len() == 2
                && route.get(1).unwrap() == "rooms"
                && bytes_start.name().into_inner() == b"room"
            {
                handle_room_declaration(bytes_start, serialization_buffer);
            } else if route.len() == 2
                && route.get(1).unwrap() == "teachers"
                && bytes_start.name().into_inner() == b"teacher"
            {
                handle_teachers_declaration(bytes_start, serialization_buffer)
            }
        }

        Event::Start(bytes_start) => {
            if route.len() == 3
                && route.get(1).unwrap() == "courses"
                && bytes_start.name().into_inner() == b"course"
            {
                handle_course_declaration(bytes_start, reader, buffer, serialization_buffer);

                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            } else if route.len() == 4
                && route.get(1).unwrap() == "solution"
                && route.get(2).unwrap() == "groups"
                && bytes_start.name().into_inner() == b"group"
            {
                handle_solution_group_declaration(
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
                handle_solution_class_declaration(
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
                handle_solution_session_declaration(
                    bytes_start,
                    reader,
                    buffer,
                    serialization_buffer,
                );
                // manually pop the route, as handle_course_declaration parse the ending XML tag
                route.pop();
            }
        }

        _ => (),
    }
}

fn handle_solution_session_declaration<R: BufRead>(
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSession, R>(reader, &node, buffer, serialization_buffer) {
        Ok(class) => {
            // debug!("new session: {:?}", class);
        }
        Err(e) => {
            error!("Error while deserializing a session from solution: {}", e);
        }
    }
}

fn handle_solution_class_declaration<R: BufRead>(
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSolutionClass, R>(reader, &node, buffer, serialization_buffer) {
        Ok(class) => {
            // debug!("new class: {:?}", class);
        }
        Err(e) => {
            error!("Error while deserializing class from solution: {}", e);
        }
    }
}
fn handle_solution_group_declaration<R: BufRead>(
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlSolutionGroup, R>(reader, &node, buffer, serialization_buffer) {
        Ok(group) => {
            // debug!("new group: {:?}", group);
        }
        Err(e) => {
            error!("Error while deserializing group from solution: {}", e);
        }
    }
}
fn handle_course_declaration<R: BufRead>(
    node: BytesStart,
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    serialization_buffer: &mut Vec<u8>,
) {
    match deserialize_element::<XmlCourse, R>(reader, &node, buffer, serialization_buffer) {
        Ok(course) => {
            // debug!("new course: {:?}", course);
        }
        Err(e) => {
            error!("Error while deserializing course: {}", e);
        }
    }
}

fn handle_room_declaration(node: BytesStart, serialization_buffer: &mut Vec<u8>) {
    match deserialize_empty_element::<XmlRoom>(node, serialization_buffer) {
        Ok(room) => {
            // debug!("new room : {:?}", room);
        }
        Err(e) => {
            error!("Error while deserializing room : {}", e);
        }
    }
}

fn handle_teachers_declaration(node: BytesStart, serialization_buffer: &mut Vec<u8>) {
    match deserialize_empty_element::<XmlTeacher>(node, serialization_buffer) {
        Ok(teacher) => {
            // debug!("new teacher: {:?}", teacher);
        }
        Err(e) => {
            error!("Error while deserializing teacher: {}", e);
        }
    }
}

fn handle_sessions_declaration(node: BytesStart) {
    let attributes = str::from_utf8(node.attributes_raw()).unwrap();
    debug!("new sessions: {attributes}");
}
fn deserialize_empty_element<T: DeserializeOwned>(
    element: BytesStart,
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    output_buffer.clear();

    let mut w = Writer::new(&mut *output_buffer);
    w.write_event(Event::Empty(element));

    return deserialize_from_buffer(output_buffer);
}

/// Returns an Utf8Error, or a DeError
fn deserialize_from_buffer<T: DeserializeOwned>(
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    let str = std::str::from_utf8(&output_buffer).map_err(|e| Box::new(e))?;

    return from_str::<T>(str).map_err(|e| Box::from(e));
}

fn deserialize_event<T: DeserializeOwned, R: BufRead>(
    event: Event,
    reader: &mut Reader<R>,
    junk_buf: &mut Vec<u8>,
    output_buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error>> {
    return match event {
        Event::Start(start_tag) => {
            deserialize_element::<T, R>(reader, &start_tag, junk_buf, output_buffer)
        }
        Event::Empty(empty_tag) => deserialize_empty_element::<T>(empty_tag, output_buffer),
        _ => return Err(Box::from("This type of event is not supported")),
    };
}

// From https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
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

fn deserialize_empty_event<'a, T: DeserializeOwned>(
    tag: BytesStart,
    output_buffer: &'a mut Vec<u8>,
) -> &'a mut Vec<u8> {
    output_buffer.clear();

    let mut w = Writer::new(&mut *output_buffer);
    w.write_event(Event::Empty(tag));

    return output_buffer;
}
