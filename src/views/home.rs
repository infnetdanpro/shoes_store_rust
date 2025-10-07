use crate::models::products::ProductsWithCategory;
use crate::models::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn home(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Html<String> {
    let products = sqlx::query(
        "
   SELECT id,
       name,
       price,
       images,
       rating,
       code,
       category_name
FROM (SELECT p.id,
             p.name,
             p.price::integer,
             p.images,
             p.rating,
             p.code,
             c.name AS category_name,
             ROW_NUMBER() OVER (
                 PARTITION BY c.id
                 ORDER BY p.created_at DESC
                 )  AS rn
      FROM product_categories pc
               JOIN products p ON p.id = pc.product_id
               JOIN categories c ON c.id = pc.category_id) t
WHERE rn <= 5;",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut map_products: HashMap<String, Vec<ProductsWithCategory>> = HashMap::new();

    for product in products {
        map_products
            .entry(product.get("category_name"))
            .or_insert_with(Vec::new)
            .push(ProductsWithCategory {
                id: product.get("id"),
                name: product.get("name"),
                price: product.get("price"),
                images: product.get("images"),
                rating: product.get("rating"),
                code: product.get("code"),
            })
    }
    let template = state.tpl_env.get_template("index.html").unwrap();
    let r = template
        .render(context!(latest_categories_products => map_products))
        .unwrap();
    Html(r)
}
