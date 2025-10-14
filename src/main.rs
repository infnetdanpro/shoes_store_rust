mod config;
mod db;
mod middlewares;
mod models;
mod repository;
mod router;
mod views;

use crate::config::config;
use crate::router::create_router;
use minijinja::Environment;
use simple_cookie::SigningKey;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[tokio::main]
async fn main() {
    let (port, addr, pool, static_files) = config().await;

    let mut env = Environment::new();
    minijinja_embed::load_templates!(&mut env);

    let signing_key = std::env::var("SIGNING_KEY")
        .map(|s| s.into_bytes())
        .unwrap_or_else(|_| vec![0; 32]);

    let app_router = create_router(
        pool,
        static_files,
        env,
        SigningKey::try_from(signing_key).unwrap(),
    );

    let tcp_listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();
    println!("Listening on http://{}:{}", addr, port);
    axum::serve(tcp_listener, app_router.into_make_service())
        .await
        .unwrap();
}
