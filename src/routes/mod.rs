mod hello_world;
mod mirror_body_string;
mod mirror_body_json;

use axum::{
    routing::{get, post},
    Router, body::Body,
};

use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;

pub fn create_routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}

