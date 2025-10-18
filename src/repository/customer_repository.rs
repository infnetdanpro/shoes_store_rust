use crate::models::customer::{NewCustomer, ProfileCustomer};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{NaiveDate, NaiveTime};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub enum CustomerError {
    Database,
    HashingError,
    MissingData(String),
}

impl std::fmt::Display for CustomerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerError::Database => write!(f, "Database error occurred"),
            CustomerError::HashingError => write!(f, "Password hashing failed"),
            CustomerError::MissingData(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl std::error::Error for CustomerError {}

impl From<sqlx::Error> for CustomerError {
    fn from(_e: sqlx::Error) -> Self {
        CustomerError::Database
    }
}

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn verify_customer(
        pool: &PgPool,
        customer_id: i64,
    ) -> Result<ProfileCustomer, CustomerError> {
        let customer = sqlx::query!(
            "SELECT id, email, first_name, last_name, date_birth, phone, city, country FROM customers WHERE id = $1", // is_enabled/is_deleted/ or something
            customer_id
        )
        .fetch_one(pool)
        .await?;

        Ok(ProfileCustomer {
            is_authenticated: true,
            id: customer.id,
            email: customer.email,
            first_name: customer
                .first_name
                .ok_or_else(|| CustomerError::MissingData("First Name is required".to_string()))?,
            last_name: customer
                .last_name
                .ok_or_else(|| CustomerError::MissingData("Last Name is required".to_string()))?,
            date_birth: NaiveDate::from(customer.date_birth),
            phone: customer
                .phone
                .ok_or_else(|| CustomerError::MissingData("phone is required".to_string()))?,
            city: customer
                .city
                .ok_or_else(|| CustomerError::MissingData("city is required".to_string()))?,
            country: customer
                .country
                .ok_or_else(|| CustomerError::MissingData("country is required".to_string()))?,
        })
    }

    pub async fn create_customer(
        pool: &PgPool,
        new_customer: NewCustomer,
    ) -> Result<i64, CustomerError> {
        let hashed_pwd =
            hash(&new_customer.password, DEFAULT_COST).map_err(|_| CustomerError::HashingError)?;
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
        .await?;

        Ok(result.id)
    }

    pub async fn get_user_by_email_password(
        pool: &PgPool,
        email: String,
        password: String,
        confirm_password: String,
    ) -> Result<ProfileCustomer, CustomerError> {
        if password != confirm_password {
            return Err(CustomerError::MissingData(
                "Passwords do not match".to_string(),
            ));
        }

        let customer = sqlx::query!("SELECT id, email, first_name, last_name, date_birth, phone, city, country, password FROM customers WHERE email = $1", email).fetch_one(pool).await?;

        let stored_password = customer
            .password
            .ok_or_else(|| CustomerError::MissingData("Password is required".to_string()))?;

        let is_same_pwd =
            verify(&password, &stored_password).map_err(|_| CustomerError::HashingError)?;

        if !is_same_pwd {
            return Err(CustomerError::MissingData("Invalid password".to_string()));
        }

        Ok(ProfileCustomer {
            is_authenticated: true,
            id: customer.id,
            email: customer.email,
            first_name: customer
                .first_name
                .ok_or_else(|| CustomerError::MissingData("First Name is required".to_string()))?,
            last_name: customer
                .last_name
                .ok_or_else(|| CustomerError::MissingData("Last Name is required".to_string()))?,
            date_birth: NaiveDate::from(customer.date_birth),
            phone: customer
                .phone
                .ok_or_else(|| CustomerError::MissingData("phone is required".to_string()))?,
            city: customer
                .city
                .ok_or_else(|| CustomerError::MissingData("city is required".to_string()))?,
            country: customer
                .country
                .ok_or_else(|| CustomerError::MissingData("country is required".to_string()))?,
        })
    }
}
