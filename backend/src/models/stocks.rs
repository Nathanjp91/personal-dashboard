use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use sqlx;
use sqlx::postgres::PgQueryResult;
use yahoo::YahooError;
use crate::schema::stocks::StockJson;
use yahoo_finance_api as yahoo;
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct StockModel {
    pub id: i32,
    pub ticker: String,
    pub amount_held: i32,
    pub last_updated: NaiveDate,
}

impl StockModel {
    pub fn new(ticker: String, amount_held: i32) -> Self {
        Self {
            id: -1,
            ticker,
            amount_held,
            last_updated: chrono::Utc::now().naive_utc().date(),
        }
    }
    pub async fn insert(&self, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"INSERT INTO stocks (ticker, amount_held, last_updated) VALUES ($1, $2, $3) RETURNING id, ticker, amount_held, last_updated"#,
            self.ticker,
            self.amount_held,
            self.last_updated
        ).fetch_one(db_pool).await
    }
    pub async fn update(&self, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"UPDATE stocks SET ticker = $1, amount_held = $2, last_updated = $3 WHERE id = $4 RETURNING id, ticker, amount_held, last_updated"#,
            self.ticker,
            self.amount_held,
            chrono::Utc::now().naive_utc().date(),
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn udpate_by_id(id: i32, stock: StockJson, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"UPDATE stocks SET ticker = $1, amount_held = $2, last_updated = $3 WHERE id = $4 RETURNING id, ticker, amount_held, last_updated"#,
            stock.ticker,
            stock.amount_held,
            chrono::Utc::now().naive_utc().date(),
            id
        ).fetch_one(db_pool).await
    }
    pub async fn delete(&self, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"DELETE FROM stocks WHERE id = $1 RETURNING id, ticker, amount_held, last_updated"#,
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn delete_by_id(id: i32, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"DELETE FROM stocks WHERE id = $1 RETURNING id, ticker, amount_held, last_updated"#,
            id
        ).fetch_one(db_pool).await
    }
    pub async fn get_all(db_pool: &sqlx::PgPool) -> Result<Vec<StockModel>, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"SELECT * FROM stocks"#
        ).fetch_all(db_pool).await
    }
    pub async fn get_all_tickers(db_pool: &sqlx::PgPool) -> Result<Vec<String>, sqlx::Error> {
        let stocks = Self::get_all(db_pool).await?;
        let mut tickers = Vec::new();
        for stock in stocks {
            tickers.push(stock.ticker);
        }
        Ok(tickers)
    }
    pub async fn get_by_id(id: i32, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"SELECT * FROM stocks WHERE id = $1"#,
            id
        ).fetch_one(db_pool).await
    }
    pub async fn get_by_ticker(ticker: String, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        sqlx::query_as!(
            StockModel,
            r#"SELECT * FROM stocks WHERE ticker = $1"#,
            ticker
        ).fetch_one(db_pool).await
    }
    pub async fn update_if_exists_or_create(&self, db_pool: &sqlx::PgPool) -> Result<StockModel, sqlx::Error> {
        let result = Self::get_by_ticker(self.ticker.clone(), db_pool).await;
        match result {
            Ok(stock) => {
                let mut new_stock = stock;
                new_stock.amount_held += self.amount_held;
                match new_stock.amount_held {
                    x if x <= 0 => new_stock.delete(db_pool).await,
                    _ => new_stock.update(db_pool).await
                }
            },
            Err(_) => self.insert(db_pool).await
        }
    }
    pub async fn delete_all(db_pool: &sqlx::PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM stocks"#,
        ).execute(db_pool).await
    }
}

pub async fn is_valid_ticker(ticker: &str) -> bool {
    let provider = yahoo::YahooConnector::new();
    let resp = provider.get_latest_quotes(ticker, "1d").await;
    match resp {
        Ok(resp) => resp.quotes().unwrap_or_default().len() > 0,
        Err(_) => false
    }
}

pub async fn valid_ticker(ticker: &str) -> Result<(), YahooError> {
    let provider = yahoo::YahooConnector::new();
    let resp = provider.get_latest_quotes(ticker, "1d")
        .await
        .map_err(|_| YahooError::FetchFailed("Ticker could not be found on yahoo finance".to_string()))?;
    if resp.quotes().unwrap_or_default().len() > 0 {
        Ok(())
    } else {
        Err(YahooError::FetchFailed("Ticker could not be found on yahoo finance".to_string()))
    }
}