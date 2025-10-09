use crate::models::state::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn get_order_by_uuid_and_customer(
    Path(order_uuid): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    println!("order_uuid: {}", order_uuid);
    let template = state.tpl_env.get_template("orders.html").unwrap();
    let r = template.render(context!()).unwrap();
    Html(r)
}
