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
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

mod modules;
mod db;

use crate::modules::constants::{CONNECTION_POOL_ERROR, SEPARATOR};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
pub fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}


// For adding authentication see https://stackoverflow.com/questions/62269278/how-can-i-make-protected-routes-in-actix-web

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

    info!("Creating pooled connection...");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Run DB migrations
    info!("Migrating database...");
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    run_db_migrations(&mut conn);

    let wrapped_pool = Data::new(pool);

    info!("Running HTTP Server...");
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