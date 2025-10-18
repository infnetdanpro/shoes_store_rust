use crate::models::customer::ProfileCustomer;
use crate::models::state::AppState;
use axum::Extension;
use axum::extract::{Path, State};
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn get_order_by_uuid_and_customer(
    Path(order_uuid): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(customer_user): Extension<ProfileCustomer>,
) -> Html<String> {
    println!("order_uuid: {}", order_uuid);// todo: create it
    let template = state.tpl_env.get_template("orders.html").unwrap();
    let r = template
        .render(context!(customer_user => customer_user))
        .unwrap();
    Html(r)
}
