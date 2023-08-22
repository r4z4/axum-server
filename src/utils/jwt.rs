use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claim {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claim {exp, iat};
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid() -> Result<bool, StatusCode> {
    todo!()
}