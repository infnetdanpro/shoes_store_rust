use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: String) -> Pool<Postgres> {
    println!("Connecting to database: {}", database_url);
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database")
}