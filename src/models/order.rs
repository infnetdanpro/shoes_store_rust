use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrderForm {
    pub(crate) product_id: i32, // uuid
    pub(crate) quantity: i32,
}

// update by order_id

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedOrder {
    pub(crate) order_id: String, // uuid
    pub(crate) product_ids: Vec<i32>,
    pub(crate) created_at: NaiveDateTime,
}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct Order {
//     pub(crate) order_id: String, // uuid
//     pub(crate) product_ids: Vec<i32>,
// }