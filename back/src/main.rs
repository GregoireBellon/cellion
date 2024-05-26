use std::error::Error;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2, sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{error, info};

mod api;
mod models;
mod xml_parsing;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/health")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("healthy");
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
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

    run_migrations(&mut pool.get().unwrap())
        .inspect(|_| info!("Migrations ran successfully"))
        .inspect_err(|e| error!("{}", e))
        .expect("The migrations failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(api::get_scope())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
