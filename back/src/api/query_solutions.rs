use actix_web::{error::ErrorBadRequest, get, web, Error as ActixError, HttpResponse, Responder};
use diesel::{result::Error as DieselError, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Serialize;

use crate::{models::db::Solution, schema, DbPool};

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

    let result: Result<FilterList, DieselError> = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let courses = schema::classes::table
            .filter(schema::classes::solution_id.eq(request_solution_id))
            .select(schema::classes::id)
            .get_results::<String>(&mut conn)?;

        let parts = schema::parts::table
            .filter(schema::parts::solution_id.eq(request_solution_id))
            .select(schema::parts::id)
            .get_results::<String>(&mut conn)?;

        let teachers = schema::teachers::table
            .filter(schema::teachers::solution_id.eq(request_solution_id))
            .select(schema::teachers::name)
            .get_results::<String>(&mut conn)?;

        let rooms = schema::rooms::table
            .filter(schema::rooms::solution_id.eq(request_solution_id))
            .select(schema::rooms::id)
            .get_results::<String>(&mut conn)?;

        let groups = schema::groups::table
            .filter(schema::groups::solution_id.eq(request_solution_id))
            .select(schema::groups::id)
            .get_results::<String>(&mut conn)?;

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
    let result: Result<Vec<Solution>, DieselError> = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        schema::solutions::table.load::<Solution>(&mut conn)
    })
    .await?;

    match result {
        Ok(filters) => Ok(HttpResponse::Ok().json(filters)),
        Err(err) => Err(ErrorBadRequest(err)),
    }
}
