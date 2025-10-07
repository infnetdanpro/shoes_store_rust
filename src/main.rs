mod config;
mod db;
mod models;
mod repository;
mod router;
mod views;

use crate::config::config;
use crate::router::create_router;
use minijinja::Environment;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[tokio::main]
async fn main() {
    let (port, addr, pool, static_files) = config().await;

    let mut env = Environment::new();
    minijinja_embed::load_templates!(&mut env);
    let app_router = create_router(pool, static_files, env);

    let tcp_listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();
    axum::serve(tcp_listener, app_router.into_make_service())
        .await
        .unwrap();
}
