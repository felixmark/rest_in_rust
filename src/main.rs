extern crate actix_web;
extern crate diesel;

use std::env;
use dotenvy::dotenv;
use log::info;
use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use r2d2::PooledConnection;

mod constants;
mod schema;
mod response;
mod notes;
mod routes;

use crate::constants::SEPARATOR;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env
    let _ = dotenv();

    // Set up logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    info!("{}", SEPARATOR);
    info!("R.I.R. started");
    info!("{}", SEPARATOR);

    // Get Database URL from .env file and set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let wrapped_pool = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(wrapped_pool.clone())
            .wrap(middleware::Logger::default())
            .service(routes::index)
            .service(routes::list)
            .service(routes::get)
            .service(routes::create)
            .service(routes::delete)
    })
    .bind(("127.0.0.1", 8010))?
    .run()
    .await
}