use crate::models::state::AppState;
use crate::views::{
    about::about, customer::get_customer_registration_page,
    customer::post_customer_registration_page, home::home, order::get_order_by_uuid_and_customer,
    products::get_product_by_code, products::get_products, products::get_products_by_category_name,
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
        .route("/order/{order_uuid}", get(get_order_by_uuid_and_customer))
        // .route("/order", post())
        .route(
            "/register",
            get(get_customer_registration_page).post(post_customer_registration_page),
        )
        // .route("/login", get(get_customer_registration_page))
        .layer(Extension(pool))
        .with_state(Arc::new(AppState { tpl_env: env }))
        .nest_service("/static", static_files)
}
