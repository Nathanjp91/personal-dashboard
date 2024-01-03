
use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::routing::{get, post};
use schema::quotes::QuoteJson;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use axum::extract::State;

use std::sync::Arc;
use serde_json::json;
mod schema;
mod handlers;
mod models;
use models::quotes::QuoteModel;
#[derive(Clone)]
pub struct AppState {
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

async fn insert_quote(
    State(app_state): State<Arc<AppState>>,
    Json(quote): Json<QuoteJson>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let db_pool = &app_state.db_pool;
    let quote_model: QuoteModel = quote.into();
    let result = quote_model.insert(db_pool).await;
    match result {
        Ok(quote) => {
            let quote: QuoteJson = quote.into();
            Ok(Json(json!(quote)).into_response())
        },
        Err(e) => {
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response())
        }
    }
}

#[tokio::main]
async fn main() {
    //get environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    
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
        .route("/", get(index))
        .route("/quotes", get(get_quotes_handler).post(insert_quote))
        .nest("/api", handlers::build_router())
        .with_state(state);

    // Run our application as a hyper server on http://localhost:8080.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}