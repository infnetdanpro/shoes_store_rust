use crate::models::state::AppState;
use crate::repository::product_repository::ProductRepository;
use axum::Extension;
use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn home(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Html<String> {
    let map_products = ProductRepository::get_latest_products_for_main(&pool, 5)
        .await
        .unwrap();
    let template = state.tpl_env.get_template("index.html").unwrap();
    let r = template
        .render(context!(latest_categories_products => map_products))
        .unwrap();
    Html(r)
}
