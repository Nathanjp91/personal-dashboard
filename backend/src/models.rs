use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx;
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct StockModel {
    pub id: i32,
    pub ticker: String,
    pub amount_held: i32,
    pub last_updated: NaiveDateTime,
}