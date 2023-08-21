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
mod create_eligible_case;
mod create_provider;
mod create_patient;
mod create_insurer;
mod create_iro;
mod custom_provider_extractor;
mod custom_patient_extractor;
mod get_eligible_case;
mod get_provider;
mod get_patient;
mod get_insurer;
mod get_iro;
mod update_provider;
mod update_patient;
mod update_insurer;
mod update_iro;
mod update_eligible_case;
mod patch_provider;

use axum::{
    routing::{get, post, put, patch},
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
use patch_provider::partial_update_provider;
use update_patient::atomic_update_patient;
use update_provider::atomic_update_provider;
use update_insurer::atomic_update_insurer;
use update_iro::atomic_update_iro;
use update_eligible_case::atomic_update_eligible_case;
use create_eligible_case::create_eligible_case;
use create_provider::create_provider;
use create_patient::create_patient;
use create_insurer::create_insurer;
use create_iro::create_iro;
use custom_provider_extractor::custom_provider_extractor;
use custom_patient_extractor::custom_patient_extractor;
use get_eligible_case::{get_eligible_case, get_all_eligible_cases};
use get_provider::{get_provider, get_all_providers};
use get_patient::{get_patient, get_all_patients};
use get_insurer::{get_insurer, get_all_insurers};
use get_iro::{get_iro, get_all_iros};

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
        // Update Routes
        .route("/provider/:provider_id", put(atomic_update_provider))
        .route("/provider/patch/:provider_id", patch(partial_update_provider))
        .route("/patient/:patient_id", put(atomic_update_patient))
        .route("/insurer/:insurer_id", put(atomic_update_insurer))
        .route("/iro/:iro_id", put(atomic_update_iro))
        .route("/eligible_case/:eligible_case_id", put(atomic_update_eligible_case))
        // Create Routes
        .route("/create_eligible_case", post(create_eligible_case))
        .route("/create_provider", post(create_provider))
        .route("/create_patient", post(create_patient))
        .route("/create_insurer", post(create_insurer))
        .route("/create_iro", post(create_iro))
        // Get routes for Entities
        .route("/custom_provider_extractor", get(custom_provider_extractor))
        .route("/custom_patient_extractor", get(custom_patient_extractor))
        .route("/get_eligible_cases", get(get_all_eligible_cases))
        .route("/get_eligible_case/:eligible_case_id", get(get_eligible_case))
        .route("/get_providers", get(get_all_providers))
        .route("/get_provider/:provider_id", get(get_provider))
        .route("/get_patients", get(get_all_patients))
        .route("/get_patient/:patient_id", get(get_patient))
        .route("/get_insurers", get(get_all_insurers))
        .route("/get_insurer/:insurer_id", get(get_insurer))
        .route("/get_iros", get(get_all_iros))
        .route("/get_iro/:iro_id", get(get_iro))
        .layer(Extension(database))
}