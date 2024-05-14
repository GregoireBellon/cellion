use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2, SqliteConnection};
mod api;
mod models;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/health")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("healthy");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(
        std::env::var("DATABASE_URL").expect("Please fill the \"DATABASE_URL\" env variable "),
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(api::get_scope())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
