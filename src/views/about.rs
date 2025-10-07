use std::sync::Arc;
use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use crate::models::state::AppState;

pub async  fn about(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = state.tpl_env.get_template("about.html").unwrap();
    let r = template.render(context!()).unwrap();
    Html(r)
}