use crate::models::customer::Customer;
use axum::Extension;
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use simple_cookie::SigningKey;
use sqlx::PgPool;

pub async fn extract_user_id_from_cookie(
    headers: HeaderMap,
    Extension(signing_key): Extension<SigningKey>,
    Extension(pool): Extension<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_user_id(&headers, &signing_key, &pool) {
        Ok(customer) => {
            req.extensions_mut().insert(customer.id);
            Ok(next.run(req).await)
        }
        Err((status, _)) => Err(status),
    }
}

fn extract_user_id(
    header_map: &HeaderMap,
    _signing_key: &SigningKey,
    _pool: &PgPool,
) -> Result<Customer, (StatusCode, String)> {
    if let Some(cookie_headers) = header_map.get("cookie") {
        let ch = cookie_headers.to_str();
        match ch {
            Ok(ch) => {
                println!("{}", ch); // TODO: parse cookie
                Ok(Customer { id: 11 }) // TODO auth 
            }
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
