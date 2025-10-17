use simple_cookie::{SigningKey, decode_cookie, encode_cookie};

pub struct AuthService;

impl AuthService {
    pub fn create_cookie_header(customer_id: i64, signing_key: &SigningKey) -> String {
        let encoded = encode_cookie(*signing_key, "customer_id", customer_id.to_le_bytes());

        let cookie_value = format!(
            "PHPSESSID={}; HttpOnly; Secure; SameSite=Strict; Max-Age=86400",
            encoded
        );
        cookie_value
    }
    pub fn parse_cookie_value(cookie_value: &str, signing_key: SigningKey) -> Result<i32, String> {
        // cookie_value = "PHPSESSID=fedkhbbkiagplcgmamicbhlgankcjgbdimhbpjifchimbbhihbbfpcdbdkebedkp"
        let cookie_pairs = cookie_value.split(';').next().unwrap();

        if let Some((name, value)) = cookie_pairs.split_once("=")
            && name.trim() == "PHPSESSID"
            && let Ok(decoded) = decode_cookie(signing_key, "customer_id", value)
        {
            return if decoded.len() == 4 {
                let bytes: [u8; 4] = decoded.try_into().unwrap();
                let customer_id = i32::from_le_bytes(bytes);
                Ok(customer_id)
            } else {
                Err("Failed to decode cookie".to_string())
            };
        }
        Err("Invalid cookie format".to_string())
    }
}
