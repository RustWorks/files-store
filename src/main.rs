extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use tracing::info;

mod config;
mod errors;
mod repositories;
mod routes;
// mod sanitize_path;
mod auth;
mod storages;

use crate::config::Config;
use crate::storages::LocalStorage;

embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::new().expect("Config Error");
    let address = config.address();

    info!(
        "Server at http://{} with local_storage_path={}",
        &address, &config.local_storage_path
    );

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
            .service(routes::upload::upload)
            .service(routes::create_directory::create_directory)
            .service(routes::get_files::get_root_files)
            .service(routes::get_files::get_files)
            .service(routes::delete_fs_node::delete_fs_node_route)
            .service(routes::download::download)
    })
    .bind(&address)?
    .run()
    .await
}
