use crate::models::customer::{AcceptEnum, NewCustomer, ProfileCustomer};
use crate::models::state::AppState;
use crate::repository::customer_repository::CustomerRepository;
use crate::services::auth::AuthService;
use axum::body::Body;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
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
) -> Response {
    let mut form_errors = HashMap::new();

    if let AcceptEnum::Off = customer.accept_all {
        form_errors.insert("accept_all", "Required field!");
    }

    let customer_id = CustomerRepository::create_customer(&pool, customer).await;

    match customer_id {
        Ok(customer_id) => {
            let mut resp = Redirect::permanent("/").into_response();
            let cookie_value = AuthService::create_cookie_header(customer_id, &signed_key);
            resp.headers_mut()
                .insert("Set-Cookie", cookie_value.parse().unwrap());
            return resp;
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

    Response::builder().status(200).body(Body::from(r)).unwrap()
}

pub async fn get_profile_customer_page(
    State(state): State<Arc<AppState>>,
    Extension(customer_user): Extension<ProfileCustomer>,
) -> Html<String> {
    let template = state.tpl_env.get_template("profile.html").unwrap();
    let r = template
        .render(context!(customer_user => customer_user))
        .unwrap();
    Html(r)
}
