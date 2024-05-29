use actix_web::{
    error::{self, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    get, post, web, Error as ActixError, HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::FilterDsl, result::Error as DieselError, ExpressionMethods, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::do_with_db,
    db::{model::Solution, schema},
    DbPool,
};

use super::service::{get_filter_list, get_sessions_with_filters};

#[derive(Serialize)]
pub struct FilterList {
    pub courses: Vec<String>,
    pub parts: Vec<String>,
    pub teachers: Vec<String>,
    pub rooms: Vec<String>,
    pub groups: Vec<String>,
}

#[get("/{solution_id}/filters")]
pub async fn get_availables_filters(
    info: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    let request_solution_id = info.into_inner();

    let result: Result<FilterList, DieselError> =
        do_with_db(pool, move |conn| get_filter_list(conn, request_solution_id)).await?;

    match result {
        Ok(filters) => Ok(HttpResponse::Ok().json(filters)),
        Err(err) => Err(ErrorBadRequest(err)),
    }
}

#[get("")]
pub async fn get_availables_solutions(
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    let result: Result<Vec<Solution>, DieselError> = do_with_db(pool, move |conn| {
        schema::solutions::table.load::<Solution>(conn)
    })
    .await?;

    match result {
        Ok(filters) => Ok(HttpResponse::Ok().json(filters)),
        Err(err) => Err(ErrorBadRequest(err)),
    }
}

#[get("/{solution_id}")]
pub async fn get_solution(
    info: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    let request_solution_id = info.into_inner();
    let result: Result<Solution, DieselError> = do_with_db(pool, move |conn| {
        schema::solutions::table
            .filter(schema::solutions::id.eq(request_solution_id))
            .get_result::<Solution>(conn)
    })
    .await?;

    match result {
        Ok(filters) => Ok(HttpResponse::Ok().json(filters)),
        Err(DieselError::NotFound) => Err(ErrorNotFound(format!(
            "Solution {} not found",
            request_solution_id
        ))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

#[derive(Deserialize)]
struct ReadInstanceBody {
    pub from: Option<String>,
    pub to: Option<String>,
    pub courses: Vec<String>,
    pub parts: Vec<String>,
    pub teachers: Vec<String>,
    pub rooms: Vec<String>,
    pub groups: Vec<String>,
}

#[post("/{solution_id}/query")]
pub async fn get_sessions(
    body: web::Json<ReadInstanceBody>,
    info: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    let request_solution_id = info.into_inner();
    let body = body.into_inner();

    let date_fmt = "%Y-%m-%dT%H:%M:%S%.f%z";
    let mut parsed_from: Option<NaiveDateTime> = None;
    let mut parsed_to: Option<NaiveDateTime> = None;

    if let Some(from) = body.from {
        let date_time = NaiveDateTime::parse_from_str(&from, date_fmt).map_err(|_| {
            error::ErrorBadRequest(format!(
                "Invalid date format for parameter 'from', expected {}",
                date_fmt
            ))
        });
        parsed_from = Some(date_time?)
    }

    if let Some(to) = body.to {
        let date_time = NaiveDateTime::parse_from_str(&to, date_fmt).map_err(|_| {
            error::ErrorBadRequest(format!(
                "Invalid date format for parameter 'to', expected {}",
                date_fmt
            ))
        });
        parsed_to = Some(date_time?)
    }

    let result = do_with_db(pool, move |conn| {
        get_sessions_with_filters(
            conn,
            request_solution_id,
            parsed_from,
            parsed_to,
            body.courses,
            body.parts,
            body.teachers,
            body.rooms,
            body.groups,
        )
    })
    .await?;

    match result {
        Ok(sessions) => Ok(HttpResponse::Ok().json(sessions)),
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
