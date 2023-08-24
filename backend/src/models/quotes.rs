use chrono::{NaiveDate, NaiveDateTime};
use sqlx;
use sqlx::postgres::PgQueryResult;
use time;
use sqlx::types::BigDecimal;
use yahoo_finance_api::time::OffsetDateTime;
use yahoo_finance_api::time::macros::datetime;
use bigdecimal::FromPrimitive;
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct QuoteModel {
    pub ticker: String,
    pub date: NaiveDate,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub close: BigDecimal,
    pub volume: i64,
}

impl QuoteModel {
    pub async fn insert(&self, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        let result = QuoteModel::get_by_ticker_date(self.ticker.clone(), self.date.clone(), db_pool).await;
        match result {
            Ok(_) => {
                return self.update(db_pool).await;
            },
            Err(_) => {}
        }
        sqlx::query_as!(
            QuoteModel,
            r#"INSERT INTO quotes ( ticker, date, open, high, low, close, volume ) VALUES ( $1, $2, $3, $4, $5, $6, $7 ) RETURNING *"#,
            self.ticker,
            self.date,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
        ).fetch_one(db_pool).await
    }
    pub async fn delete(&self, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"DELETE FROM quotes WHERE ( ticker = $1 AND date = $2 ) RETURNING *"#,
            self.ticker,
            self.date
        ).fetch_one(db_pool).await
    }
    pub async fn populate_ticker(ticker: String, db_pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        
        let end = OffsetDateTime::now_utc();
        let start = datetime!(1970-01-01 0:00 UTC);
        let result = yahoo_finance_api::YahooConnector::new().get_quote_history(&ticker, start, end).await;
        match result {
            Ok(result) => {
                let quotes = result.quotes();
                println!("Quotes: {:?}", quotes);
                for quote in quotes.unwrap_or_default() {
                    let date = NaiveDateTime::from_timestamp_opt(quote.timestamp as i64, 0);
                    if date.is_none() {
                        continue;
                    }
                    let quote = QuoteModel {
                        ticker: ticker.clone(),
                        date: date.unwrap().date(),
                        open: BigDecimal::from_f64(quote.open).unwrap_or(BigDecimal::from_f64(0.0).unwrap()),
                        high: BigDecimal::from_f64(quote.high).unwrap_or(BigDecimal::from_f64(0.0).unwrap()),
                        low: BigDecimal::from_f64(quote.low).unwrap_or(BigDecimal::from_f64(0.0).unwrap()),
                        close: BigDecimal::from_f64(quote.close).unwrap_or(BigDecimal::from_f64(0.0).unwrap()),
                        volume: quote.volume as i64,
                    };
                    let result = quote.insert(db_pool).await;
                    match result {
                        Ok(_) => {println!("Inserted quote: {}", quote.date);},
                        Err(_) => {println!("Failed to insert quote: {}", quote.date);}
                    }
                }
                return Ok(());
            },
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(sqlx::Error::RowNotFound);
            }
        }
    }
    pub async fn update(&self, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"UPDATE quotes SET open = $3, high = $4, low = $5, close = $6, volume = $7 WHERE ( ticker = $1 AND date = $2 ) RETURNING *"#,
            self.ticker,
            self.date,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
        ).fetch_one(db_pool).await
    }
    pub async fn get_all(db_pool: &sqlx::PgPool) -> Result<Vec<QuoteModel>, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes"#
        ).fetch_all(db_pool).await
    }
    pub async fn get_by_ticker_date(ticker: String, date: NaiveDate, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE ticker = $1 AND date = $2"#,
            ticker,
            date
        ).fetch_one(db_pool).await
    }
    pub async fn get_date_range(ticker: String, start: NaiveDate, end: NaiveDate, db_pool: &sqlx::PgPool) -> Result<Vec<QuoteModel>, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE ticker = $1 AND date >= $2 AND date <= $3"#,
            ticker,
            start,
            end
        ).fetch_all(db_pool).await
    }
    pub async fn get_all_date_range(start: NaiveDate, end: NaiveDate, db_pool: &sqlx::PgPool) -> Result<Vec<QuoteModel>, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE date >= $1 AND date <= $2"#,
            start,
            end
        ).fetch_all(db_pool).await
    }
    pub async fn get_closest_date(ticker: String, date: NaiveDate, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE ticker = $1 AND date <= $2 ORDER BY date DESC LIMIT 1"#,
            ticker,
            date
        ).fetch_one(db_pool).await
    }
    pub async fn delete_all(db_pool: &sqlx::PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM quotes"#,
        ).execute(db_pool).await
    }
}