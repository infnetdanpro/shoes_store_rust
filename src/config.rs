use dotenv::dotenv;
use crate::db::create_pool;
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;

pub async fn config() -> (String, String, Pool<Postgres>, ServeDir) {
    dotenv().ok();
    
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/postgres".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let pool = create_pool(database_url).await;
    let static_files = ServeDir::new("./assets");
    (port, addr, pool, static_files)
}
