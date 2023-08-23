use crate::{
    database::user::{self, Model, Entity as User},
    utils::app_error::*,

};
use axum::{
    headers::{authorization::Bearer, Authorization},
    extract::{Extension, Json},
    http::StatusCode,
};
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
    let expires_in = Duration::days(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claim {exp, iat};
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claim>(token, &key, &Validation::new(Algorithm::HS256))
        .map_err(|error| {
            match error.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(StatusCode::UNAUTHORIZED, "Your session has expired. Please login again."),
                _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong. Please try again."),
            }
        })?;
    Ok(true)
}