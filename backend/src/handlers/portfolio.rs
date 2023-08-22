
use actix_web::{get, web, HttpResponse, Responder};
use crate::{
    AppState,
    models::StockModel,
    schema::{StockJson, PortfolioJson}
};

#[get("/portfolio")]
pub async fn calculate_porfolio(state: web::Data<AppState>) -> impl Responder {
    let result = StockModel::get_all(&state.db_pool).await;
    match result {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get stocks")
    }
    let mut stocks = result.unwrap();
    let mut stocks_json: Vec<StockJson> = Vec::new();
    for stock in stocks.iter_mut() {
        let mut stock_json = StockJson::from_model(stock.clone());
        stock_json.calculate_value().await;
        stocks_json.push(stock_json);
    }
    let total = stocks_json.iter().fold(0.0, |acc, stock| {
        acc + stock.value.unwrap_or_default() * stock.amount_held as f64
    });
    let portfolio = PortfolioJson {
        stocks: stocks_json,
        total: total
    };
    HttpResponse::Ok().json(portfolio)
}

pub fn configure_portfolio(config: &mut web::ServiceConfig) {
    config.service(calculate_porfolio);
}