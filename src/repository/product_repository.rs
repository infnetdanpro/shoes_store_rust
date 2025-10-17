use crate::models::products::{FullProduct, Product, ProductsWithCategory};
use sqlx::{PgPool, Row};
use std::collections::HashMap;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn get_latest_products_for_main(
        pool: &PgPool,
        limit: i32,
    ) -> Result<HashMap<String, Vec<ProductsWithCategory>>, sqlx::Error> {
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
WHERE rn <= $1;",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

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
        Ok(map_products)
    }
    pub async fn get_products_with_pagination(
        pool: &PgPool,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Product>, i64), sqlx::Error> {
        let (product_results, count_results) = tokio::join!(
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
            .fetch_all(pool),
            sqlx::query(
                r#"select count(*) as count
            from products p
            join product_categories pc on p.id = pc.product_id
            join categories c on pc.category_id = c.id
            where c.is_active = true;"#,
            )
            .fetch_one(pool)
        );

        let products = product_results?;
        let cnt_products = count_results?;
        let count: i64 = cnt_products.get("count");
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
        Ok((ctx_products, count))
    }

    pub async fn get_products_by_category_with_pagination(
        category_name: &str,
        pool: &PgPool,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Product>, i64, Option<String>, Option<String>), sqlx::Error> {
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
            .bind(category_name)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool),
            sqlx::query(
                r#"
select
    count(*) as count
from products p
join product_categories pc on p.id = pc.product_id
join categories c on pc.category_id = c.id
where c.is_active = true and c.name = $1;"#
            )
            .bind(category_name)
            .fetch_one(pool)
        );

        let products = products_result?;
        let cnt_products = count_result?;
        let count: i64 = cnt_products.get("count");

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

        Ok((ctx_products, count, category_name, category_description))
    }

    pub async fn get_product_by_code(
        code: &str,
        pool: &PgPool,
    ) -> Result<FullProduct, sqlx::Error> {
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
        .bind(code)
        .fetch_one(pool)
        .await?;

        Ok(FullProduct {
            id: product.get("id"),
            name: product.get("name"),
            description: product.get("description"),
            price: product.get("price"),
            rating: product.get("rating"),
            code: product.get("code"),
            images: product.get("images"),
        })
    }
}
