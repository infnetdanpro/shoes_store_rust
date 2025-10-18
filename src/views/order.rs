use crate::models::customer::ProfileCustomer;
use crate::models::order::NewOrderForm;
use crate::models::state::AppState;
use crate::repository::order_repository::OrderRepository;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{Extension, Form};
use minijinja::context;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn post_add_product_to_cart(
    Extension(pool): Extension<PgPool>,
    Extension(customer_user): Extension<ProfileCustomer>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewOrderForm>,
) -> Html<String> {
    let template = state.tpl_env.get_template("orders.html").unwrap();

    let result =
        OrderRepository::create_order(&pool, customer_user.id, form.product_id, form.quantity)
            .await;
    match result {
        Ok(products_order) => {
            println!("products_order: {:?}", products_order);
            let template = state.tpl_env.get_template("orders.html").unwrap();
            let r = template
                .render(context!(customer_user => customer_user, products_order => products_order))
                .unwrap();
            Html(r)
        }
        Err(e) => {
            tracing::error!(
                "Error creating order: {:?}. Form: {:?}. User: {:?}",
                e,
                form,
                customer_user
            );
            let r = template
                .render(context!(customer_user => customer_user, is_error => true ))
                .unwrap();
            Html(r)
        }
    }
}

pub async fn get_order_by_uuid_and_customer(
    Path(order_uuid): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(customer_user): Extension<ProfileCustomer>,
) -> Html<String> {
    tracing::info!("order_uuid: {}", order_uuid);
    let template = state.tpl_env.get_template("orders.html").unwrap();
    let r = template
        .render(context!(customer_user => customer_user))
        .unwrap();
    Html(r)
}
