use crate::models::products::{CategoryProducts, Pagination};
use crate::models::state::AppState;
use crate::repository::product_repository::ProductRepository;
use axum::Extension;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use minijinja::context;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_products(
    pagination: Query<Pagination>,
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Html<String> {
    let pagination = pagination.0;
    let limit = 9;
    let current_page = if pagination.page < 1 {
        1
    } else {
        pagination.page
    };
    let offset = (current_page - 1) * limit;

    let (ctx_products, count) =
        ProductRepository::get_products_with_pagination(&pool, offset, limit)
            .await
            .unwrap();

    let total_pages: f64 = (count as f64) / (limit as f64);
    let mut page_numbers = Vec::new();
    let start_page = std::cmp::max(1, current_page - 2);
    let end_page = std::cmp::min(total_pages.ceil() as i64, current_page + 2);

    for page in start_page..=end_page {
        page_numbers.push(page);
    }

    let path_url = "products/";

    let template = state.tpl_env.get_template("products.html").unwrap();
    let r = template
        .render(context!(
            url => path_url,
            products => ctx_products,
            current_page => current_page,
            total_pages => total_pages.ceil(),
            page_numbers => page_numbers,
            has_prev => current_page > 1,
            has_next => current_page < (total_pages.ceil()) as i64,
            prev_page => current_page - 1,
            next_page => current_page + 1,
        ))
        .unwrap();
    Html(r)
}

pub async fn get_products_by_category_name(
    Path(category_name): Path<CategoryProducts>,
    pagination: Query<Pagination>,
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Html<String> {
    let pagination = pagination.0;
    let limit = 9;
    let current_page = if pagination.page < 1 {
        1
    } else {
        pagination.page
    };
    let offset = (current_page - 1) * limit;

    let (ctx_products, count, category_name, category_description) =
        ProductRepository::get_products_by_category_with_pagination(
            &category_name.category_name,
            &pool,
            offset,
            limit,
        )
        .await
        .unwrap();

    let total_pages: f64 = (count as f64) / (limit as f64);
    let mut page_numbers = Vec::new();
    let start_page = std::cmp::max(1, current_page - 2);
    let end_page = std::cmp::min(total_pages.ceil() as i64, current_page + 2);

    for page in start_page..=end_page {
        page_numbers.push(page);
    }

    let template = state.tpl_env.get_template("products.html").unwrap();
    let mut path_url = String::from("category/");
    let cat_name = category_name.clone().expect("men");
    path_url.push_str(&cat_name.as_str()); // TODO: clone?

    let r = template
        .render(context!(
            url => path_url,
            category_name => category_name,
            category_description => category_description,
            products => ctx_products,
            current_page => current_page,
            total_pages => total_pages.ceil(),
            page_numbers => page_numbers,
            has_prev => current_page > 1,
            has_next => current_page < (total_pages.ceil()) as i64,
            prev_page => current_page - 1,
            next_page => current_page + 1,
        ))
        .unwrap();
    Html(r)
}

pub async fn get_product_by_code(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Html<String> {
    let ctx_product = ProductRepository::get_product_by_code(&code, &pool)
        .await
        .unwrap();
    let template = state.tpl_env.get_template("single-product.html").unwrap();
    let r = template.render(context!(product => ctx_product)).unwrap();
    Html(r)
}
