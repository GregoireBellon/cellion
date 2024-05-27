use std::{fs::File, io::BufReader, path::Path, str};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error as actix_error, post, web, Error as ActixError, HttpResponse, Responder};
use chrono::Utc;
use diesel::ExpressionMethods;
use log::{debug, warn};
use quick_xml::events::Event;
use serde::Serialize;

use crate::{
    api::solution_service::SolutionInserter,
    models::schema,
    xml_parsing::{
        reader::{self, EventHandlingError, Router, XmlParser, XmlRouting, XmlRoutingError},
        types::{
            XmlCalendar, XmlCourse, XmlRoom, XmlSession, XmlSolutionClass, XmlSolutionGroup,
            XmlStudent, XmlTeacher,
        },
    },
    DbPool,
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

    enum BlockError {
        DbError(diesel::result::Error),
        ExtractFileError(ExtractFileError),
    }

    web::block(move || -> Result<UploadResult, BlockError> {
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
        .map_err(|e| BlockError::DbError(e))?;

        debug!("Solution inserted");

        extract_file(payload.file.file.path(), &mut solution_inserter)
            .map_err(|e| BlockError::ExtractFileError(e))?;

        debug!("File extracted ");

        let db_result = solution_inserter
            .insert_all_into_db()
            .map(|inserted| UploadResult {
                id: solution_inserter.solution_id(),
                row_inserted: inserted,
            })
            .map_err(|e| BlockError::DbError(e))?;

        return Ok(db_result);
    })
    .await?
    .map(|result| HttpResponse::Ok().json(result))
    .map_err(|e| match e {
        BlockError::DbError(dbe) => actix_error::ErrorFailedDependency(format!(
            "Error while interacting with the database : {:?}",
            dbe
        )),
        BlockError::ExtractFileError(ExtractFileError::FileOpeningError(fe)) => {
            actix_error::ErrorInternalServerError(format!(
                "Error while opening the file : {:?}",
                fe
            ))
        }
        BlockError::ExtractFileError(ExtractFileError::RoutingError(re)) => match re {
            XmlRoutingError::RoutingError(e) => actix_error::ErrorInternalServerError(format!(
                "Error while parsing the file : {:?}",
                e
            )),
            XmlRoutingError::HandlingError(reader::EventHandlingError::DeserializationError(
                de,
            )) => {
                actix_error::ErrorBadRequest(format!("Error while deserializing a tag: {:?}", de))
            }
            XmlRoutingError::HandlingError(reader::EventHandlingError::UnsupportedEventType) => {
                actix_error::ErrorInternalServerError(format!(
                    "Error while trying to deserialize an Event....the code is buggy :("
                ))
            }
        },
    })
}

enum ExtractFileError {
    RoutingError(XmlRoutingError<EventHandlingError>),
    FileOpeningError(quick_xml::Error),
}

fn extract_file(
    file: &Path,
    solution_inserter: &mut SolutionInserter,
) -> Result<(), ExtractFileError> {
    let mut router: Router<BufReader<File>, EventHandlingError, SolutionInserter> = vec![
        XmlRouting {
            route: vec!["timetabling", "rooms", "room"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |room: XmlRoom| {
                        context.add_room(room);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "teachers", "teacher"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |teacher: XmlTeacher| {
                        context.add_teacher(teacher);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "students", "student"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |student: XmlStudent| {
                        context.add_student(student);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "courses", "course"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |course: XmlCourse| {
                        context.add_course(course);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "solution", "sessions", "session"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |session: XmlSession| {
                        context
                            .add_session(session)
                            .inspect_err(|e| {
                                warn!("Error while adding the session : {:?}", e);
                            })
                            .ok();
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "solution", "groups", "group"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |group: XmlSolutionGroup| {
                        context.add_solution_group(group);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "solution", "classes", "class"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |class: XmlSolutionClass| {
                        context.add_solution_class(class);
                    });
                },
            ),
        },
        XmlRouting {
            route: vec!["timetabling", "calendar"],
            handler: Box::from(
                |event: Event,
                 parser: &mut XmlParser<BufReader<File>>,
                 context: &mut SolutionInserter| {
                    return parser.handle_event(event, |calendar: XmlCalendar| {
                        context
                            .add_calendar(calendar)
                            .inspect_err(|e| warn!("Error while adding the calendar: {:?}", e))
                            .ok();
                    });
                },
            ),
        },
    ];

    let mut parser =
        XmlParser::from_file(file).map_err(|e| ExtractFileError::FileOpeningError(e))?;

    parser
        .walk_buffer(&mut router, solution_inserter)
        .map_err(|e| ExtractFileError::RoutingError(e))?;

    return Ok(());
}
