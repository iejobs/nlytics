use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type DbPool = Pool<Postgres>;

pub async fn init(database_url: &str) -> DbPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect to database")
}

pub async fn run_migrations(db: &DbPool) -> () {
    sqlx::migrate!("./migrations")
        .run(db)
        .await
        .expect("Failed to run migrations")
}