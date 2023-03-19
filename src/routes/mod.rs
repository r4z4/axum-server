mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod mirror_user_agent;
mod mirror_custom_header;
mod path_var;
mod query_params;
mod middleware_message;
mod always_errors;
mod returns_201;
mod get_json;
mod validate_with_serde;
mod create_provider;
mod custom_provider_extractor;
mod custom_patient_extractor;
mod get_provider;

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
use sea_orm::{DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};
use middleware_message::middleware_message;
use always_errors::always_errors;
use returns_201::returns_201;
use get_json::get_json;
use validate_with_serde::validate_with_serde;
use create_provider::create_provider;
use custom_provider_extractor::custom_provider_extractor;
use custom_patient_extractor::custom_patient_extractor;
use get_provider::{get_provider, get_all_providers};

#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
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
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", post(get_json))
        .route("/validate_with_serde", post(validate_with_serde))
        .route("/create_provider", post(create_provider))
        .route("/custom_provider_extractor", get(custom_provider_extractor))
        .route("/custom_patient_extractor", get(custom_patient_extractor))
        .route("/get_providers", get(get_all_providers))
        .route("/get_provider/:provider_id", get(get_provider))
        .layer(Extension(database))
}