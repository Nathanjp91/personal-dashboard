
use actix_web::{get, web, HttpResponse, Responder};
use crate::{
    AppState,
    models::stocks::StockModel,
    schema::stocks::{StockJson, PortfolioJson}
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
    let mut total = 0.0;
    for stock in stocks.iter_mut() {
        let mut stock_json = StockJson::from_model(stock.clone());
        stock_json.calculate_value().await;
        total += stock_json.value.unwrap_or_default() * stock_json.amount_held as f64;
        stocks_json.push(stock_json);
    }
    let portfolio = PortfolioJson {
        stocks: stocks_json,
        total: total
    };
    HttpResponse::Ok().json(portfolio)
}

pub fn configure_portfolio(config: &mut web::ServiceConfig) {
    config.service(calculate_porfolio);
}