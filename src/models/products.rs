use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProductsWithCategory {
    // main page (latest products)
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) price: i32,
    pub(crate) images: Json<Vec<String>>,
    pub(crate) rating: i32,
    pub(crate) code: String,
}

fn default_page() -> i64 {
    1
}
#[derive(Deserialize, Serialize)]
pub struct Pagination {
    // > /products + /products?page=2
    #[serde(default = "default_page")]
    pub(crate) page: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryProducts {
    pub(crate) category_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Product {
    // render product in products list
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) price: i32,
    pub(crate) rating: i32,
    pub(crate) code: String,
    pub(crate) images: Json<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullProduct {
    // render product
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) price: i32,
    pub(crate) rating: i32,
    pub(crate) code: String,
    pub(crate) images: Json<Vec<String>>,
}
