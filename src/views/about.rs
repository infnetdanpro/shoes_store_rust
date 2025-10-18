use crate::models::customer::ProfileCustomer;
use crate::models::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

pub async fn about(
    State(state): State<Arc<AppState>>,
    Extension(customer_user): Extension<ProfileCustomer>,
) -> Html<String> {
    let template = state.tpl_env.get_template("about.html").unwrap();
    let r = template
        .render(context!(customer_user => customer_user))
        .unwrap();
    Html(r)
}
