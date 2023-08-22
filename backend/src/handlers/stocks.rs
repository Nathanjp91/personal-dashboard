use crate::{
    models::StockModel,
    schema::StockJson,
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};



#[post("/stocks")]
pub async fn add_stock(stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
    let stock = StockModel::new(stock.ticker.clone(), stock.amount_held);
    let result = stock.update_if_exists_or_create(&state.db_pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(result.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add stock")
    }
}

#[patch("/stocks/id/{id}")]
pub async fn update_stocks(id: web::Path<i32>, stock: web::Json<StockJson>, state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::udpate_by_id(id.into_inner(), stock.into_inner(), &state.db_pool).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(result.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update stock")
    }
}

#[get("/stocks")]
pub async fn get_stocks(state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::get_all(&state.db_pool).await;
    match result {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get stocks")
    }
}

#[delete("/stocks/id/{id}")]
pub async fn delete_stocks(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::delete_by_id(id.into_inner(), &state.db_pool).await;
    match result {
        Ok(stock) => HttpResponse::Ok().json(stock),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete stock")
    }
}

#[get("/stocks/id/{id}")]
pub async fn get_stock_by_id(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::get_by_id(id.into_inner(), &state.db_pool).await;
    match result {
        Ok(stock) => HttpResponse::Ok().json(stock),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get stock")
    }
}

#[get("/stocks/ticker/{ticker}")]
pub async fn get_stock_by_ticker(ticker: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::get_by_ticker(ticker.into_inner(), &state.db_pool).await;
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

