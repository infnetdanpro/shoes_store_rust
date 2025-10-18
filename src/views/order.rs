use crate::models::customer::ProfileCustomer;
use crate::models::order::NewOrderForm;
use crate::models::state::AppState;
use crate::repository::order_repository::OrderRepository;
use crate::repository::product_repository::ProductRepository;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{Extension, Form};
use minijinja::context;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_list_orders(
    Extension(pool): Extension<PgPool>,
    Extension(customer_user): Extension<ProfileCustomer>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let template = state.tpl_env.get_template("list-orders.html").unwrap();

    let result = OrderRepository::check_not_finished_order_products(&pool, customer_user.id).await;
    match result {
        Ok(order_products) => {
            let mut orders_sums: HashMap<String, i32> =
                HashMap::with_capacity(order_products.len());
            let mut order_statuses: HashMap<String, Option<String>> =
                HashMap::with_capacity(order_products.len());

            for (order_id, products) in &order_products {
                let mut sum = 0;
                for product in products {
                    sum += &product.product_price;
                    order_statuses.insert(order_id.clone(), product.order_status.clone());
                }
                orders_sums.insert(order_id.clone(), sum);
            }

            let r = template
                .render(context!(customer_user => customer_user, order_products => order_products, orders_sums => orders_sums, order_statuses => order_statuses))
                .unwrap();
            Html(r)
        }
        Err(e) => {
            tracing::error!(
                "Error retrieving orders: {:?}. User: {:?}",
                e,
                customer_user
            );
            let r = template
                .render(context!(customer_user => customer_user, is_error => true))
                .unwrap();
            Html(r)
        }
    }
}

pub async fn post_add_product_to_cart(
    Extension(pool): Extension<PgPool>,
    Extension(customer_user): Extension<ProfileCustomer>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewOrderForm>,
) -> Html<String> {
    let template = state.tpl_env.get_template("orders.html").unwrap();

    // retrieve product sum
    let product_result = ProductRepository::get_product_by_id(form.product_id, &pool).await;

    match product_result {
        Ok(product) => {
            let result = OrderRepository::create_order(
                &pool,
                customer_user.id,
                form.product_id,
                form.quantity,
                product.price,
            )
            .await;
            match result {
                Ok(products_order) => {
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
        Err(e) => {
            tracing::error!(
                "Error retrieving product: {:?}. Form: {:?}. User: {:?}",
                e,
                form,
                customer_user
            );
            let r = template
                .render(context!(customer_user => customer_user, is_error => true ))
                .unwrap();
            return Html(r);
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
