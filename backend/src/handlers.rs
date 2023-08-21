use crate::{
    models::StockModel,
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize)]
pub struct StockJson {
    id: Option<i32>,
    ticker: String,
    amount_held: i32,
    last_updated: Option<NaiveDateTime>
}

#[post("/stocks")]
pub async fn add_stock(stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
    let result: Result<_, _> = sqlx::query_as! {
        StockModel,
        r#"INSERT INTO stocks (ticker, amount_held, last_updated) VALUES ($1, $2, $3) RETURNING id, ticker, amount_held, last_updated"#,
        stock.ticker,
        stock.amount_held,
        chrono::Utc::now().naive_utc()
    }.fetch_one(&state.db_pool).await;
    let result = result.map(|stock| StockJson {
        id: Some(stock.id),
        ticker: stock.ticker,
        amount_held: stock.amount_held,
        last_updated: Some(stock.last_updated)
    });
    match result {
        Ok(_) => HttpResponse::Ok().json(result.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add stock")
    }
}

#[patch("/stocks/id/{id}")]
pub async fn update_stocks(id: web::Path<i32>, stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
    let result: Result<_, _> = sqlx::query_as! {
        StockModel,
        r#"UPDATE stocks SET ticker = $1, amount_held = $2, last_updated = $3 WHERE id = $4 RETURNING id, ticker, amount_held, last_updated"#,
        stock.ticker,
        stock.amount_held,
        chrono::Utc::now().naive_utc(),
        id.into_inner()
    }.fetch_one(&state.db_pool).await;
    let result = result.map(|stock| StockJson {
        id: Some(stock.id),
        ticker: stock.ticker,
        amount_held: stock.amount_held,
        last_updated: Some(stock.last_updated)
    });
    match result {
        Ok(_) => HttpResponse::Ok().json(result.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update stock")
    }
}

#[get("/stocks")]
pub async fn get_stocks(state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as! {
        StockJson,
        r#"SELECT * FROM stocks"#
    }.fetch_all(&state.db_pool).await;
    match result {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get stocks")
    }
}

#[delete("/stocks/id/{id}")]
pub async fn delete_stocks(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as! {
        StockJson,
        r#"DELETE FROM stocks WHERE id = $1 RETURNING id, ticker, amount_held, last_updated"#,
        id.into_inner()
    }.fetch_one(&state.db_pool).await;
    match result {
        Ok(stock) => HttpResponse::Ok().json(stock),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete stock")
    }
}

#[get("/stocks/id/{id}")]
pub async fn get_stock_by_id(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as! {
        StockJson,
        r#"SELECT * FROM stocks WHERE id = $1"#,
        id.into_inner()
    }.fetch_one(&state.db_pool).await;
    match result {
        Ok(stock) => HttpResponse::Ok().json(stock),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get stock")
    }
}

#[get("/stocks/ticker/{ticker}")]
pub async fn get_stock_by_ticker(ticker: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as! {
        StockJson,
        r#"SELECT * FROM stocks WHERE ticker = $1"#,
        ticker.into_inner()
    }.fetch_all(&state.db_pool).await;
    match result {
        Ok(stock) => HttpResponse::Ok().json(stock),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get stock")
    }
}

pub fn configure_stocks(config: &mut web::ServiceConfig) {
    config.service(add_stock);
    config.service(get_stocks);
    config.service(get_stock_by_id);
    config.service(get_stock_by_ticker);
    config.service(delete_stocks);
    config.service(update_stocks);
}

