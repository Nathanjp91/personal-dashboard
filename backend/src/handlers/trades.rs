use crate::{
    models::stocks::is_valid_ticker,
    models::trades::TradeModel,
    schema::trades::TradeJson,
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};


#[post("/trades")]
pub async fn add_trade(state: web::Data<AppState>, trade: web::Json<TradeJson>) -> impl Responder {
    if !(is_valid_ticker(&trade.ticker).await) {
        return HttpResponse::BadRequest().body("Invalid ticker");
    }
    let result: TradeModel = trade.into_inner().into();
    let result = result.insert(&state.db_pool).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(TradeJson::from(result.unwrap())),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add trade")
    }
}

#[get("/trades")]
pub async fn get_trades(state: web::Data<AppState>) -> impl Responder {
    let result = TradeModel::get_all(&state.db_pool).await;
    match result {
        Ok(trades) => HttpResponse::Ok().json(trades.into_iter().map(|trade| TradeJson::from(trade)).collect::<Vec<TradeJson>>()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get trades")
    }
}
pub fn configure_trades(config: &mut web::ServiceConfig) {
    config.service(add_trade);
    config.service(get_trades);
}