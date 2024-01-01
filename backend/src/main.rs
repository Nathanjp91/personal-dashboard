
use axum::Json;
use axum::response::IntoResponse;
use axum::{handler::Handler};
use axum::{http::StatusCode};
use sqlx::postgres::{PgPoolOptions};
use dotenv::dotenv;
use axum::extract::State;
use std::path::Path;
use std::sync::Arc;
use serde_json::json;
mod schema;

mod models;
use models::quotes::QuoteModel;
#[derive(Clone)]
struct AppState {
    db_pool: sqlx::postgres::PgPool,
}

async fn index() -> &'static str {
    "Hello, Index!"
}

async fn get_quotes_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let db_pool = &app_state.db_pool;
    let tickers = QuoteModel::get_tickers(db_pool).await;
    match tickers {
        Ok(tickers) => {
            Ok(Json(json!({ "tickers": tickers })).into_response())
        },
        Err(e) => {
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response())
        }
    }
}

fn generate_docker_url() -> String {
    let postgres_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    let postgres_url = format!("postgres://{}:{}@{}:{}/{}", postgres_user, postgres_password, postgres_host, postgres_port, postgres_db);
    std::env::set_var("DATABASE_URL", postgres_url.clone());
    postgres_url
}

#[tokio::main]
async fn main() {
    //get environment variables
    dotenv().ok();

    let database_url = generate_docker_url();

    
    // setup database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let state = Arc::new(AppState {
        db_pool: db_pool.clone(),
    });

    // Build our application with a single route.
    let app = axum::Router::new()
    .route("/",
        axum::routing::get(index))
    .route("/quotes", 
    axum::routing::get(get_quotes_handler))
    .with_state(state);

    // Run our application as a hyper server on http://localhost:8080.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}