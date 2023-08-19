mod routes;
mod database;
use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database);
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}