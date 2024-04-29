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

mod modules;
mod db;

use crate::modules::constants::SEPARATOR;

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
            .service(modules::routes::index)
            .service(modules::routes::list)
            .service(modules::routes::get)
            .service(modules::routes::create)
            .service(modules::routes::delete)
    })
    .bind(("0.0.0.0", 8010))?
    .run()
    .await
}