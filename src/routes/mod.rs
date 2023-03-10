mod hello_world;

use axum::{
    routing::{get, post, put},
    Router, body::Body,
};

use hello_world::hello_world;

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/", get(hello_world))
}

