use crate::{
    models::stocks::{StockModel, is_valid_ticker, valid_ticker},
    schema::stocks::{StockJson, ErrorJson, ErrorType},
    AppState,
};
use std::{sync::Arc, string};
use axum::Router;
use axum::{routing::{get, post}, response::IntoResponse, http::StatusCode};
use axum::Json;
use serde_json::json;
use axum::extract::State;

pub async fn add_stock(
    State(app_state): State<Arc<AppState>>,
    Json(stock): Json<StockJson>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    valid_ticker(&stock.ticker).await.map_err(internal_error)?;
    let stock = StockModel::new(stock.ticker.clone(), stock.amount_held);
    let result = stock.update_if_exists_or_create(&app_state.db_pool).await;

    match result {
        Ok(stock) => {
            let stock: StockJson = stock.into();
            Ok(Json(json!(stock)))
        },
        Err(e) => {
            Err(internal_error(e))
        }
    }
}

pub fn internal_error<E>(e: E) -> (StatusCode, Json<serde_json::Value>)
where
    E: std::error::Error,
{
    match e {
        YahooError => {
            (StatusCode::BAD_REQUEST, Json(json!({ "error": "Collecting from Yahoo Finance failed" })))
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

// #[post("/stocks")]
// pub async fn add_stock(stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
//     if !is_valid_ticker(&stock.ticker).await {
//         return HttpResponse::BadRequest().json(ErrorJson::with_message(ErrorType::InvalidTicker, "Ticker could not be found on yahoo finance".to_string()));
//     }
//     let stock = StockModel::new(stock.ticker.clone(), stock.amount_held);
//     let result = stock.update_if_exists_or_create(&state.db_pool).await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json(result.unwrap()),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// #[patch("/stocks/id/{id}")]
// pub async fn update_stocks(id: web::Path<i32>, stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
//     let result = StockModel::udpate_by_id(id.into_inner(), stock.into_inner(), &state.db_pool).await;
//     match result {
//         Ok(_) => HttpResponse::Ok().json(result.unwrap()),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// #[get("/stocks")]
// pub async fn get_stocks(state: web::Data<AppState>) -> impl Responder {
//     let result = StockModel::get_all(&state.db_pool).await;
//     match result {
//         Ok(stocks) => HttpResponse::Ok().json(stocks),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// #[delete("/stocks/id/{id}")]
// pub async fn delete_stocks(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
//     let result = StockModel::delete_by_id(id.into_inner(), &state.db_pool).await;
//     match result {
//         Ok(stock) => HttpResponse::Ok().json(stock),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// #[get("/stocks/id/{id}")]
// pub async fn get_stock_by_id(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
//     let result = StockModel::get_by_id(id.into_inner(), &state.db_pool).await;
//     match result {
//         Ok(stock) => HttpResponse::Ok().json(stock),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// #[get("/stocks/ticker/{ticker}")]
// pub async fn get_stock_by_ticker(ticker: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
//     let result = StockModel::get_by_ticker(ticker.into_inner(), &state.db_pool).await;
//     match result {
//         Ok(stock) => HttpResponse::Ok().json(stock),
//         Err(message) => HttpResponse::InternalServerError().json(ErrorJson::with_message(ErrorType::DatabaseError, message.to_string()))
//     }
// }

// pub fn configure_stocks(config: &mut web::ServiceConfig) {
//     config.service(add_stock);
//     config.service(get_stocks);
//     config.service(get_stock_by_id);
//     config.service(get_stock_by_ticker);
//     config.service(delete_stocks);
//     config.service(update_stocks);
// }


pub fn build_router() -> Router<Arc<AppState>> {
    Router::new().route("/stocks", post(add_stock))
}