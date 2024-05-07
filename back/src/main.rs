use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

pub mod models;
pub mod schema;

#[derive(Serialize)]
struct Resp {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().json(Resp {
        message: "hello world".to_string(),
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
