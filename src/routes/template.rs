use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;
use axum::{
    extract::{FromRef, Path},
    response::{IntoResponse, TemplateEngine},
    routing::get,
    Router, Server,
};

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}
type AppEngine = Engine<Handlebars<'static>>;
async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the key
    Key(key): Key,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(key, engine, person)
}


pub async fn template_render() -> String {
    // Set up the Handlebars engine with the same route paths as the Axum router
    // let mut hbs = Handlebars::new();
    // hbs.register_template_string("/template_render/:name", "<h1>Hello HandleBars!</h1><p>{{name}}</p>")
    //     .owned();
    "Hello World from Routes".to_owned()

    // let app = Router::new()
    //     .route("/template_render/:name", get(get_name))
    //     // Create the application state
    //     .with_state(AppState {
    //         engine: Engine::from(hbs),
    //     });

    // Server::bind(&([127, 0, 0, 1], 8080).into())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}