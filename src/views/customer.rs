use crate::models::customer::{AcceptEnum, CustomerLoginPostForm, NewCustomer, ProfileCustomer};
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
    let template = state.tpl_env.get_template("registration.html").unwrap();

    let customer_id = CustomerRepository::create_customer(&pool, customer).await;

    match customer_id {
        Ok(customer_id) => {
            let cookie_value = AuthService::create_cookie_header(customer_id, &signed_key);
            let r = template.render(context!(is_register_ok => true)).unwrap();

            let mut resp = Response::builder().status(200).body(Body::from(r)).unwrap();
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

pub async fn get_customer_login_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = state.tpl_env.get_template("login.html").unwrap();
    let r = template.render(context!()).unwrap();
    Html(r)
}

pub async fn post_customer_login_page(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(signed_key): Extension<SigningKey>,
    Form(form): Form<CustomerLoginPostForm>,
) -> Result<Response, Html<String>> {
    // post form perform

    let template = state.tpl_env.get_template("login.html").unwrap();

    let result = CustomerRepository::get_user_by_email_password(
        &pool,
        form.email,
        form.password,
        form.confirm_password,
    )
    .await;
    match result {
        Ok(customer_user) => {
            let r = template.render(context!(is_login_ok => true)).unwrap();
            let cookie_value = AuthService::create_cookie_header(customer_user.id, &signed_key);
            let mut resp = Response::builder().status(200).body(Body::from(r)).unwrap();
            resp.headers_mut()
                .insert("Set-Cookie", cookie_value.parse().unwrap());
            Ok(resp)
        }
        Err(_) => {
            let mut form_errors = HashMap::new();
            form_errors.insert("error", "User not found or password is incorrect");
            let r = template
                .render(context!(form_errors => form_errors))
                .unwrap();
            Err(Html(r))
        }
    }
}

pub async fn logout_customer() -> Response {
    let mut resp = Redirect::permanent("/").into_response();
    resp.headers_mut().insert("Set-Cookie", "PHPSESSID=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; expires=Thu, 01 Jan 1970 00:00:00 GMT".parse().unwrap());
    resp
}
