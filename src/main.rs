#[macro_use]
extern crate actix_web;
extern crate diesel;

use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use r2d2::PooledConnection;
use log::LevelFilter;
use log::info;
mod constants;
mod schema;
mod response;
mod notes;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    println!("HELLO CALLED.");
    HttpResponse::Ok().body("Oh hello there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    //simple_logging::log_to_file("test.log", LevelFilter::Info).expect("Could not create logger.");
    //simple_logging::log_to_stderr(LevelFilter::Debug);
    
    info!("R.I.R. started.");

    // set up database connection pool
    let database_url = constants::DATABASE_URL;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let wrapped_pool = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(wrapped_pool.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(notes::list)
            .service(notes::get)
            .service(notes::create)
            .service(notes::delete)
    })
    .bind(("127.0.0.1", 8010))?
    .run()
    .await
}