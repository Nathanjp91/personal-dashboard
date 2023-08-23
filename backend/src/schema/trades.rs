use std::convert::{From, Into};

use bigdecimal::ToPrimitive;
use chrono::NaiveDate;
use crate::models::trades::{Country, TradeType, TradeModel};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use bigdecimal::FromPrimitive;
#[derive(Deserialize, Serialize)]
pub struct TradeJson {
    pub id: Option<i32>,
    pub ticker: String,
    pub amount: i32,
    pub date: NaiveDate,
    pub country: Country,
    pub price: f64,
    pub trade_type: TradeType,
}

impl From<TradeModel> for TradeJson {
    fn from(model: TradeModel) -> Self {
        Self {
            id: Some(model.id),
            ticker: model.ticker,
            amount: model.amount,
            date: model.date,
            country: model.country,
            price: model.price.to_f64().unwrap_or(0.0),
            trade_type: model.trade_type,
        }
    }
}

impl Into<TradeModel> for TradeJson {
    fn into(self) -> TradeModel {
        TradeModel {
            id: self.id.unwrap_or(-1),
            ticker: self.ticker,
            amount: self.amount,
            date: self.date,
            country: self.country,
            price: BigDecimal::from_f64(self.price).unwrap_or(BigDecimal::from_f64(0.0).unwrap()),
            trade_type: self.trade_type,
        }
    }
}