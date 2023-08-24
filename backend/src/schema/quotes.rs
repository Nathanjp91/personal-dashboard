use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::models::quotes::QuoteModel;
use std::convert::{From, Into};

use bigdecimal::{ToPrimitive, FromPrimitive};
#[derive(Deserialize, Serialize)]
pub struct QuoteJson {
    pub ticker: String,
    pub date: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

impl From<QuoteModel> for QuoteJson {
    fn from(model: QuoteModel) -> Self {
        Self {
            ticker: model.ticker,
            date: model.date,
            open: model.open.to_f64().unwrap_or(0.0),
            high: model.high.to_f64().unwrap_or(0.0),
            low: model.low.to_f64().unwrap_or(0.0),
            close: model.close.to_f64().unwrap_or(0.0),
            volume: model.volume.to_i64().unwrap_or(0),
        }
    }
}
impl Into<QuoteModel> for QuoteJson {
    fn into(self) -> QuoteModel {
        QuoteModel {
            ticker: self.ticker,
            date: self.date,
            open: FromPrimitive::from_f64(self.open).unwrap_or(FromPrimitive::from_f64(0.0).unwrap()),
            high: FromPrimitive::from_f64(self.high).unwrap_or(FromPrimitive::from_f64(0.0).unwrap()),
            low: FromPrimitive::from_f64(self.low).unwrap_or(FromPrimitive::from_f64(0.0).unwrap()),
            close: FromPrimitive::from_f64(self.close).unwrap_or(FromPrimitive::from_f64(0.0).unwrap()),
            volume: FromPrimitive::from_i64(self.volume).unwrap_or(FromPrimitive::from_i64(0).unwrap()),
        }
    }
}
