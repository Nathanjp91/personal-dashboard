pub mod stocks;
pub mod portfolio;
use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    stocks::configure_stocks(config);
    portfolio::configure_portfolio(config);
}