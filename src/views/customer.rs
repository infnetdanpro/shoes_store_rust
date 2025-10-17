use crate::models::customer::{AcceptEnum, NewCustomer, ProfileCustomer};
use crate::models::state::AppState;
use crate::repository::customer_repository::CustomerRepository;
use crate::services::auth::AuthService;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::{Extension, Form};
use minijinja::context;
use simple_cookie::SigningKey;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_customer_registration_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = state.tpl_env.get_template("registration.html").unwrap();
    let r = template.render(context!()).unwrap();
    Html(r)
}

pub async fn post_customer_registration_page(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(signed_key): Extension<SigningKey>,
    Form(customer): Form<NewCustomer>,
) -> impl IntoResponse {
    let mut form_errors = HashMap::new();

    if let AcceptEnum::Off = customer.accept_all {
        form_errors.insert("accept_all", "Required field!");
    }

    let customer_id = CustomerRepository::create_customer(&pool, customer).await;

    let mut headers = HeaderMap::new();
    match customer_id {
        Ok(customer_id) => {
            let cookie_value = AuthService::create_cookie_header(customer_id, &signed_key);
            headers.insert("Set-Cookie", cookie_value.parse().unwrap());
        }
        Err(e) => {
            println!("Error register NewCustomer: {:?}", e);
            form_errors.insert(
                "error",
                "Error register customer user, please send this to the support",
            );
        }
    }

    let template = state.tpl_env.get_template("registration.html").unwrap();
    let r = template
        .render(context!(form_errors => form_errors))
        .unwrap();

    (headers, Html(r))
}

pub async fn get_profile_customer_page(
    State(state): State<Arc<AppState>>,
    Extension(_authed_customer): Extension<ProfileCustomer>,
) -> Html<String> {
    let template = state.tpl_env.get_template("index.html").unwrap();
    let r = template.render(context!()).unwrap();
    Html(r)
}
