pub mod stocks;
pub mod portfolio;
pub mod trades;
pub mod admin;
pub mod quotes;
pub mod modelling;
use std::sync::Arc;
use axum::Router;

use crate::AppState;

pub fn build_router() -> Router<Arc<AppState>> {
    let stocks = stocks::build_router();
    Router::new().merge(stocks)
}