use std::future::Future;

use actix_web::{web, Scope};
use diesel::r2d2::{self, ManageConnection, PooledConnection};

pub mod dto;
mod query;
mod solution;

pub fn get_scope() -> Scope {
    actix_web::web::scope("/solutions")
        .service(solution::controller::post_route)
        .service(query::controller::get_availables_filters)
        .service(query::controller::get_availables_solutions)
        .service(query::controller::get_solution)
        .service(query::controller::get_sessions)
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
