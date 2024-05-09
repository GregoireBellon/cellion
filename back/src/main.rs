use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

mod api;
mod models;
mod schema;

#[get("/health")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("healthy");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::publish_solution::post_route)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
