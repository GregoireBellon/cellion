use actix_web::{error::ErrorBadRequest, get, web, Error as ActixError, HttpResponse, Responder};
use diesel::{result::Error as DieselError, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        do_with_db,
        query_service::{get_filter_list, get_sessions_with_filters},
    },
    models::{db::Solution, schema},
    DbPool,
};

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

#[derive(Deserialize)]
struct ReadInstanceBody {
    pub courses: Vec<String>,
    pub parts: Vec<String>,
    pub teachers: Vec<String>,
    pub rooms: Vec<String>,
    pub groups: Vec<String>,
}

#[get("/{solution_id}/query")]
pub async fn get_sessions(
    body: web::Json<ReadInstanceBody>,
    info: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ActixError> {
    let request_solution_id = info.into_inner();
    let body = body.into_inner();

    let result = do_with_db(pool, move |conn| {
        get_sessions_with_filters(
            conn,
            request_solution_id,
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
