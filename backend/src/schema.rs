use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::models::StockModel;
use yahoo_finance_api as yahoo;

#[derive(Deserialize, Serialize)]
pub struct StockJson {
    pub id: Option<i32>,
    pub ticker: String,
    pub amount_held: i32,
    pub last_updated: Option<NaiveDateTime>,
    pub value: Option<f64>
}
impl StockJson {
    pub fn from_model(model: StockModel) -> Self {
        Self {
            id: Some(model.id),
            ticker: model.ticker,
            amount_held: model.amount_held,
            last_updated: Some(model.last_updated),
            value: None
        }
    }
    pub async fn calculate_value(&mut self) {
        let provider = yahoo::YahooConnector::new();
        let resp = provider.get_latest_quotes(&self.ticker, "1d").await;
        match resp {
            Ok(resp) => {
                self.value = Some(resp.quotes().unwrap_or_default()[0].open);
            },
            Err(_) => {
                self.value = Some(0.0);
            }
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct PortfolioJson {
    pub stocks: Vec<StockJson>,
    pub total: f64,
}