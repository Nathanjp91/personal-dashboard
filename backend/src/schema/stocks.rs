use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::models::stocks::StockModel;
use yahoo_finance_api as yahoo;
use strum_macros::{EnumString, Display};

#[derive(Deserialize, Serialize)]
pub struct StockJson {
    pub id: Option<i32>,
    pub ticker: String,
    pub amount_held: i32,
    pub last_updated: Option<NaiveDate>,
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

#[derive(Deserialize, Serialize)]
pub struct ErrorJson {
    pub error: ErrorType,
    pub message: Option<String> 
}
#[derive(Deserialize, Serialize, EnumString, Display, Clone, Copy)]
pub enum ErrorType {
    InvalidTicker,
    InvalidFireRate,
    InvalidReturns,
    InvalidExpenses,
    InvalidMonthlyInvestment,
    DatabaseError,
}

impl ErrorJson {
    pub fn default(error: ErrorType) -> Self {
        Self {
            error: error,
            message: None
        }
    }
    pub fn with_message(error: ErrorType, message: String) -> Self {
        Self {
            error: error,
            message: Some(message)
        }
    }
}