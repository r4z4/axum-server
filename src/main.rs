use axum_server::run;
use dotenvy::dotenv;
// use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let database_uri = dotenv!("DATABASE_URL");
    // This is e.g. for SQLite. So, here, uri = just the path to the DB file
    // let connection = Database::connect("/Users/me/.myapp/data.db?mode=rwc").await?;
    let database_uri = "sqlite://ExtRev";
    run(database_uri).await
}


/*
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
*/
