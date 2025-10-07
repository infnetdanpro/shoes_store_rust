use crate::models::products::{CategoryProducts, FullProduct, Pagination, Product};
use crate::models::state::AppState;
use axum::Extension;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use minijinja::context;
use sqlx::{PgPool, Row};
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

    let (products_result, count_result) = tokio::join!(
        sqlx::query(
            r#"select
    p.id,
    p.name,
    p.price::integer,
    p.rating,
    p.code,
    p.images
from products p
         join product_categories pc on p.id = pc.product_id
        join categories c on pc.category_id = c.id
where c.is_active = true
order by p.id desc
offset $1
limit $2;"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(&pool),
        sqlx::query(
            r#"select count(*) as count
            from products p
            join product_categories pc on p.id = pc.product_id
            join categories c on pc.category_id = c.id
            where c.is_active = true;"#,
        )
        .fetch_one(&pool)
    );

    let products = products_result.unwrap();
    let cnt_products = count_result.unwrap();

    let mut ctx_products: Vec<Product> = Vec::with_capacity(products.len());

    for product in &products {
        ctx_products.push(Product {
            id: product.get("id"),
            name: product.get("name"),
            price: product.get("price"),
            rating: product.get("rating"),
            code: product.get("code"),
            images: product.get("images"),
        })
    }

    let count: i64 = cnt_products.get("count");
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

    let (products_result, count_result) = tokio::join!(
        sqlx::query(
            r#"
select
    p.id,
    p.name,
    p.price::integer,
    p.rating,
    p.code,
    p.images,
    c.name as "category_name",
    c.description as "category_description"
from products p
         join product_categories pc on p.id = pc.product_id
        join categories c on pc.category_id = c.id
where c.is_active = true and c.name = $1
order by p.id desc
offset $2
limit $3;
    "#,
        )
        .bind(&category_name.category_name)
        .bind(offset)
        .bind(limit)
        .fetch_all(&pool),
        sqlx::query(
            r#"
select
    count(*) as count
from products p
join product_categories pc on p.id = pc.product_id
join categories c on pc.category_id = c.id
where c.is_active = true and c.name = $1;"#
        )
        .bind(&category_name.category_name)
        .fetch_one(&pool)
    );

    let products = products_result.unwrap();
    let cnt_products = count_result.unwrap();

    let mut ctx_products: Vec<Product> = Vec::with_capacity(products.len());

    let (category_name, category_description) = if let Some(first_product) = products.first() {
        (
            first_product.get("category_name"),
            first_product.get("category_description"),
        )
    } else {
        (None::<String>, None::<String>)
    };

    for product in &products {
        ctx_products.push(Product {
            id: product.get("id"),
            name: product.get("name"),
            price: product.get("price"),
            rating: product.get("rating"),
            code: product.get("code"),
            images: product.get("images"),
        });
    }

    let count: i64 = cnt_products.get("count");
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
    let product = sqlx::query(
        "
select
    id,
    name,
    description,
    price::integer,
    rating,
    code,
    images
from products where code = $1;",
    )
    .bind(&code)
    .fetch_one(&pool)
    .await
    .unwrap();

    let ctx_product = FullProduct {
        id: product.get("id"),
        name: product.get("name"),
        description: product.get("description"),
        price: product.get("price"),
        rating: product.get("rating"),
        code: product.get("code"),
        images: product.get("images"),
    };
    let template = state.tpl_env.get_template("single-product.html").unwrap();
    let r = template.render(context!(product => ctx_product)).unwrap();
    Html(r)
}
