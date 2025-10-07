use crate::models::state::AppState;
use crate::views::{
    about::about, home::home, products::get_product_by_code, products::get_products,
    products::get_products_by_category_name,
};
use axum::routing::get;
use axum::{Extension, Router};
use minijinja::Environment;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn create_router(pool: PgPool, static_files: ServeDir, env: Environment<'static>) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/products", get(get_products))
        .route(
            "/category/{category_name}",
            get(get_products_by_category_name),
        )
        .route("/product/{code}", get(get_product_by_code))
        .layer(Extension(pool))
        .with_state(Arc::new(AppState { tpl_env: env }))
        .nest_service("/static", static_files)
}
