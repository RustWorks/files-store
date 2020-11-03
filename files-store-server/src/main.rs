extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, rt, App, HttpServer};
use dotenv::dotenv;
use tracing::info;
use users::AuthConfig;

mod config;
mod errors;
mod jobs;
mod repositories;
mod routes;
mod storages;
mod users_routes;

use crate::config::Config;
use crate::jobs::bin_cleaner_job::BinCleanerActor;
use crate::jobs::thumbnail_job::ThumbnailActor;
use crate::storages::LocalStorage;

embed_migrations!("./migrations");

async fn create_server() -> Result<(), std::io::Error> {
    let config = Config::new().expect("Config Error");
    let address = config.address();
    let assets = config.assets();

    info!(
        "Server at http://{} with local_storage_path={} and assets={}",
        &address, &config.local_storage_path, assets
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

    let sqlx_pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("creating pool error");

    let local_storage = LocalStorage::new(&config.local_storage_path)
        .await
        .expect("Local storage path error");

    let bin_cleaner_actor = BinCleanerActor::start(sqlx_pool.clone(), local_storage.clone());

    let thumbnail_actor = ThumbnailActor::start(sqlx_pool.clone(), local_storage.clone());

    let _ = HttpServer::new(move || {
        App::new()
            .data(sqlx_pool.clone())
            .data(config.clone())
            .data(local_storage.clone())
            .data(thumbnail_actor.clone())
            .data(bin_cleaner_actor.clone())
            .data(AuthConfig::new(config.secret_key.clone()))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .configure(users_routes::init)
            .service(routes::bin_cleanup::bin_cleanup_route)
            .service(routes::upload::upload)
            .service(routes::create_directory::create_directory)
            .service(routes::get_files::get_root_files)
            .service(routes::get_files::get_files)
            .service(routes::get_thumbnail::get_thumbnail_route)
            .service(routes::move_fs_node::move_fs_node_route)
            .service(routes::move_fs_node_to_bin::move_fs_node_to_bin_route)
            .service(routes::download::download)
            .service(Files::new("/", &assets).index_file("index.html"))
    })
    .bind(&address)?
    .run()
    .await;
    Ok(())
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let mut system = rt::System::new("file-store");

    let server = create_server();

    system.block_on(server)
}
