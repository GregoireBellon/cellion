use std::future::Future;

use actix_web::{web, Scope};
use diesel::r2d2::{self, ManageConnection, PooledConnection};

mod query_controller;
mod query_service;
mod solution_controller;
mod solution_service;
mod solutions_dto;
mod xml_types;

pub fn get_scope() -> Scope {
    actix_web::web::scope("/solutions")
        .service(solution_controller::post_route)
        .service(query_controller::get_availables_filters)
        .service(query_controller::get_availables_solutions)
        .service(query_controller::get_sessions)
}

pub fn do_with_db<F, R, M>(
    pool: web::Data<r2d2::Pool<M>>,
    f: F,
) -> impl Future<Output = Result<R, actix_web::error::BlockingError>>
where
    F: FnOnce(&mut PooledConnection<M>) -> R + Send + 'static,
    R: Send + 'static,
    M: ManageConnection,
{
    return web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        return f(&mut conn);
    });
}
