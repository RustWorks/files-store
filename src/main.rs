extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

mod config;
mod download;
mod errors;
mod files_repository;
mod repository_erros;
mod sanitize_path;
mod storage;
mod upload;
mod uploaded_file;

use crate::config::Config;
use crate::storage::LocalStorage;

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = Config::new().expect("Error config");
    let address = format!("{}:{}", config.host, config.port);

    let database_url = config.database_url.clone();
    actix_web::web::block(move || {
        use diesel::prelude::*;
        let connection = diesel::pg::PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to the database"));
        embedded_migrations::run(&connection)
    })
    .await
    .expect("Migration error");

    let sqlx_pool = sqlx::PgPool::new(&config.database_url)
        .await
        .expect("creating pool error");

    let local_storage = LocalStorage::new(&config.local_storage_path)
        .await
        .expect("Local storage path error");

    HttpServer::new(move || {
        App::new()
            .data(sqlx_pool.clone())
            .data(config.clone())
            .data(local_storage.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(upload::upload)
            .service(download::download)
    })
    .bind(address)?
    .run()
    .await
}