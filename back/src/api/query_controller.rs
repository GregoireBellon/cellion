use actix_web::{error::ErrorBadRequest, get, web, Error as ActixError, HttpResponse, Responder};
use diesel::{result::Error as DieselError, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    api::{do_with_db, query_service::get_sessions_with_filters},
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

    let result: Result<FilterList, DieselError> = do_with_db(pool, move |conn| {
        let courses = schema::classes::table
            .filter(schema::classes::solution_id.eq(request_solution_id))
            .select(schema::classes::id)
            .get_results::<String>(conn)?;

        let parts = schema::parts::table
            .filter(schema::parts::solution_id.eq(request_solution_id))
            .select(schema::parts::id)
            .get_results::<String>(conn)?;

        let teachers = schema::teachers::table
            .filter(schema::teachers::solution_id.eq(request_solution_id))
            .select(schema::teachers::name)
            .get_results::<String>(conn)?;

        let rooms = schema::rooms::table
            .filter(schema::rooms::solution_id.eq(request_solution_id))
            .select(schema::rooms::id)
            .get_results::<String>(conn)?;

        let groups = schema::groups::table
            .filter(schema::groups::solution_id.eq(request_solution_id))
            .select(schema::groups::id)
            .get_results::<String>(conn)?;

        Ok(FilterList {
            courses: courses,
            parts: parts,
            teachers: teachers,
            rooms: rooms,
            groups: groups,
        })
    })
    .await?;

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
