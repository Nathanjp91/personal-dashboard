use crate::{
    models::stocks::is_valid_ticker,
    schema::stocks::{StockJson, ErrorJson, ErrorType},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};


pub fn configure_trades(config: &mut web::ServiceConfig) {
    config.service(add_stock);
    config.service(update_stocks);
    config.service(get_stocks);
    config.service(delete_stocks);
    config.service(get_stock_by_id);
}