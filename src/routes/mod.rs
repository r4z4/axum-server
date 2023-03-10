mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod mirror_user_agent;
mod mirror_custom_header;
mod path_var;
mod query_params;
mod middleware_message;

use axum::{
    routing::{get, post},
    Router, body::Body, http::Method, Extension,
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_var::path_var;
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use tower_http::cors::{Any, CorsLayer};
use middleware_message::middleware_message;

#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello from shared data".to_owned()
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_var/:id", get(path_var))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
}

