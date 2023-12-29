
use axum::{response::Html, routing::get, Router, handler::Handler};
use sqlx::postgres::{PgPoolOptions};
use dotenv::dotenv;
struct AppState {
    db_pool: sqlx::postgres::PgPool,
}

#[tokio::main]
async fn main() {
    //get environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Build our application with a single route.
    let app = axum::Router::new().route("/",
        axum::routing::get(|| async { "Test, ing!" }));

    // setup database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    
    let state = AppState {
        db_pool: db_pool,
    };
    
    // Run our application as a hyper server on http://localhost:8080.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap().with_state(state);
}