use crate::models::customer::{Customer, ProfileCustomer};
use crate::repository::customer_repository::CustomerRepository;
use crate::services::auth::AuthService;
use axum::Extension;
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use simple_cookie::SigningKey;
use sqlx::PgPool;

pub async fn optional_customer(
    headers: HeaderMap,
    Extension(signing_key): Extension<SigningKey>,
    Extension(pool): Extension<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    match extract_user_id(&headers, signing_key) {
        Ok(customer) => match CustomerRepository::verify_customer(&pool, customer.id).await {
            Ok(customer) => {
                req.extensions_mut().insert(customer);
                Ok(next.run(req).await)
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
        },
        Err((_, _)) => {
            req.extensions_mut().insert(ProfileCustomer {
                is_authenticated: false,
                id: 0,
                email: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                date_birth: Default::default(),
                phone: "".to_string(),
                city: "".to_string(),
                country: "".to_string(),
            });
            Ok(next.run(req).await)
        }
    }
}

pub async fn redirect_if_authed(
    headers: HeaderMap,
    Extension(signing_key): Extension<SigningKey>,
    req: Request,
    next: Next,
) -> Response {
    let result = extract_user_id(&headers, signing_key);
    match result {
        Ok(_) => {
            let mut res = next.run(req).await;
            res.headers_mut().insert("Location", "/".parse().unwrap());
            *res.status_mut() = StatusCode::PERMANENT_REDIRECT;
            res
        }
        Err(_) => next.run(req).await,
    }
}

pub async fn extract_user_id_from_cookie(
    headers: HeaderMap,
    Extension(signing_key): Extension<SigningKey>,
    Extension(pool): Extension<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    match extract_user_id(&headers, signing_key) {
        Ok(customer) => match CustomerRepository::verify_customer(&pool, customer.id).await {
            Ok(customer) => {
                req.extensions_mut().insert(customer);
                Ok(next.run(req).await)
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
        },
        Err((status, _)) => Err((
            status,
            String::from("Failed to extract user id from cookie"),
        )),
    }
}

fn extract_user_id(
    header_map: &HeaderMap,
    signing_key: SigningKey,
) -> Result<Customer, (StatusCode, String)> {
    if let Some(cookie_headers) = header_map.get("cookie") {
        let ch = cookie_headers.to_str();
        match ch {
            Ok(ch) => match AuthService::parse_cookie_value(ch, signing_key) {
                Ok(id) => Ok(Customer { id }),
                Err(e) => Err((
                    StatusCode::UNAUTHORIZED,
                    format!("Failed to parse cookie header: {}", e),
                )),
            },
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse cookie header: {}", e),
            )),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            "Cookie header is missing".to_string(),
        ))
    }
}
