use chrono::NaiveDate;
use sqlx;
use sqlx::types::BigDecimal;
use strum_macros::{EnumString, Display};
use serde::{Deserialize, Serialize};
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct TradeModel {
    pub id: i32,
    pub ticker: String,
    pub amount: i32,
    pub date: NaiveDate,
    pub country: Country,
    pub price: BigDecimal,
    pub trade_type: TradeType,
}
#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone, Copy)]
pub enum TradeType {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone, Copy)]
pub enum Country {
    US,
    CA,
    UK,
    AU,
}

impl std::convert::From<std::string::String> for Country {
    fn from(s: std::string::String) -> Self {
        match s.to_uppercase().as_str() {
            "US" => Country::US,
            "CA" => Country::CA,
            "UK" => Country::UK,
            "AU" => Country::AU,
            _ => Country::AU,
        }
    }
}
impl std::convert::From<std::string::String> for TradeType {
    fn from(s: std::string::String) -> Self {
        match s.to_lowercase().as_str() {
            "buy" => TradeType::Buy,
            "sell" => TradeType::Sell,
            _ => TradeType::Buy,
        }
    }
}

impl TradeModel {
    pub async fn insert(&self, db_pool: &sqlx::PgPool) -> Result<TradeModel, sqlx::Error> {
        sqlx::query_as!(
            TradeModel,
            r#"INSERT INTO trades_history (ticker, amount, date, country, price, trade_type) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#,
            self.ticker,
            self.amount,
            self.date,
            self.country.to_string(),
            self.price,
            self.trade_type.to_string()
        ).fetch_one(db_pool).await
    }
    pub async fn update(&self, db_pool: &sqlx::PgPool) -> Result<TradeModel, sqlx::Error> {
        sqlx::query_as!(
            TradeModel,
            r#"UPDATE trades_history SET ticker = $1, amount = $2, date = $3, country = $4, price = $5, trade_type = $6 WHERE id = $7 RETURNING *"#,
            self.ticker,
            self.amount,
            self.date,
            self.country.to_string(),
            self.price,
            self.trade_type.to_string(),
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn delete(&self, db_pool: &sqlx::PgPool) -> Result<TradeModel, sqlx::Error> {
        sqlx::query_as!(
            TradeModel,
            r#"DELETE FROM trades_history WHERE id = $1 RETURNING *"#,
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn get_all(db_pool: &sqlx::PgPool) -> Result<Vec<TradeModel>, sqlx::Error> {
        sqlx::query_as!(
            TradeModel,
            r#"SELECT * FROM trades_history"#
        ).fetch_all(db_pool).await
    }
    pub async fn get_all_date_range(start: NaiveDate, end: NaiveDate, db_pool: &sqlx::PgPool) -> Result<Vec<TradeModel>, sqlx::Error> {
        sqlx::query_as!(
            TradeModel,
            r#"SELECT * FROM trades_history WHERE date >= $1 AND date <= $2"#,
            start,
            end
        ).fetch_all(db_pool).await
    }
}