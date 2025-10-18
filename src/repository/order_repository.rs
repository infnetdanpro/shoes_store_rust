use crate::models::order::CreatedOrder;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct OrderRepository;

impl OrderRepository {
    pub async fn create_order(
        pool: &PgPool,
        customer_id: i64,
        product_id: i32,
        quantity: i32,
    ) -> Result<Vec<CreatedOrder>, Error> {
        let order_uuid = Self::create_order_uuid(pool, customer_id).await?;
        if order_uuid.len() == 0 {
            panic!("Order uuid is empty");
        }
        println!("order_uuid: {}", order_uuid);

        let order_products =
            Self::create_order_products_with_order_uuid(pool, order_uuid, product_id, quantity)
                .await?;
        // order_products: [17, 17]

        // retrieve product info by product_ids
        let mut result = Vec::with_capacity(order_products.len());
        for _ in order_products {
            result.push(CreatedOrder {
                order_id: "".to_string(),
                product_ids: vec![],
                created_at: Default::default(),
            })
        }
        Ok(result)
    }
    pub async fn create_order_uuid(pool: &PgPool, customer_id: i64) -> Result<String, Error> {
        // step 1 - insert into orders_product
        let result = sqlx::query!(
            "INSERT INTO orders (customer_id) VALUES ($1) returning id::varchar",
            customer_id
        )
        .fetch_one(pool)
        .await?;
        Ok(result.id.unwrap_or_else(|| "".to_string())) // kostyl'!
    }
    pub async fn create_order_products_with_order_uuid(
        // step 2 - insert into orders with customer_id
        pool: &PgPool,
        order_uuid: String,
        product_id: i32,
        quantity: i32,
    ) -> Result<Vec<i32>, Error> {
        let mut order_uuids: Vec<Uuid> = Vec::with_capacity(quantity as usize);
        let mut product_ids: Vec<i32> = Vec::with_capacity(quantity as usize);
        for _ in 0..quantity {
            order_uuids.push(order_uuid.clone().parse().unwrap());
        }
        for _ in 0..quantity {
            product_ids.push(product_id);
        }

        // https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts
        let result = sqlx::query!("INSERT INTO orders_product (order_id, product_id) SELECT * FROM UNNEST($1::uuid[], $2::int[]) RETURNING product_id", &order_uuids, &product_ids).fetch_all(pool).await?;
        let mut result_vec = Vec::with_capacity(result.len());
        for row in result {
            result_vec.push(row.product_id.unwrap_or_else(|| 0)); //what we do with zero product ids?
        }
        Ok(result_vec)
    }
}
// pub async fn get_product_by_order_uuid() {}
// pub async fn update_comment_by_order_id() {}
