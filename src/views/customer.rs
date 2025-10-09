use crate::models::customer::{AcceptEnum, NewCustomer};
use crate::models::state::AppState;
use crate::repository::customer_repository::CustomerRepository;
use axum::extract::State;
use axum::response::Html;
use axum::{Extension, Form};
use minijinja::context;
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
    Form(customer): Form<NewCustomer>,
) -> Html<String> {
    println!("{:?}", customer);
    let mut form_errors = HashMap::new();
    match customer.accept_all {
        AcceptEnum::Off => {
            // field/error
            form_errors.insert("accept_all", "Required field!");
        }
        _ => {}
    }

    let customer_id = CustomerRepository::create_customer(&pool, customer).await;

    match customer_id {
        Ok(_) => {}
        Err(e) => {
            println!("Error register NewCustomer: {}", e);
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
    Html(r)
}
