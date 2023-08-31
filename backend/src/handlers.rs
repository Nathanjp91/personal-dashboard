pub mod stocks;
pub mod portfolio;
pub mod trades;
pub mod admin;
pub mod quotes;
pub mod modelling;

use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    quotes::configure_quotes(config);
    stocks::configure_stocks(config);
    portfolio::configure_portfolio(config);
    trades::configure_trades(config);
    admin::configure_admin(config);
}