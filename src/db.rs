use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_diesel_pool(database_url: impl Into<String>) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
