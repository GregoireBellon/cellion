use actix_web::Scope;

mod publish_service;
mod publish_solution;
mod query_solutions;
mod xml_types;

pub fn get_scope() -> Scope {
    actix_web::web::scope("/solutions")
        .service(publish_solution::post_route)
        .service(query_solutions::get_availables_filters)
        .service(query_solutions::get_availables_solutions)
}
