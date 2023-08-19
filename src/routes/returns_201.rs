use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}