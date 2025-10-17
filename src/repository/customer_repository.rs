use crate::models::customer::{NewCustomer, ProfileCustomer};
use bcrypt::{DEFAULT_COST, hash};
use chrono::NaiveTime;
use sqlx::{Error, PgPool};

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn verify_customer(
        pool: &PgPool,
        customer_id: i32,
    ) -> Result<ProfileCustomer, Error> {
        let result = sqlx::query!(
            "SELECT id, email, first_name, last_name FROM customers WHERE id = $1", // is_enabled/is_deleted/ or something
            customer_id as i64
        )
        .fetch_one(pool)
        .await;
        match result {
            Ok(customer) => Ok(ProfileCustomer {
                id: customer.id,
                email: customer.email,
                first_name: customer.first_name.expect("Empty First Name"),
                last_name: customer.last_name.expect("Empty Last Name"),
            }),
            Err(e) => Err(e),
        }
    }
    pub async fn create_customer(pool: &PgPool, new_customer: NewCustomer) -> Result<i32, Error> {
        let hashed_pwd = hash(&new_customer.password, DEFAULT_COST).unwrap();
        let date_time = new_customer
            .date_birth
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        let result = sqlx::query!(
            "INSERT INTO customers (
                       email,
                       date_birth,
                       country,
                       city,
                       first_name,
                       last_name,
                       phone,
                       password
               ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id;",
            new_customer.email,
            date_time,
            new_customer.country,
            new_customer.city,
            new_customer.first_name,
            new_customer.last_name,
            new_customer.phone,
            hashed_pwd
        )
        .fetch_one(pool)
        .await;

        match result {
            Ok(result) => Ok(result.id as i32),
            Err(e) => Err(e),
        }
    }
}
