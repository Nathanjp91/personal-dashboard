use chrono::NaiveDate;
use sqlx;
use sqlx::types::BigDecimal;
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct QuoteModel {
    pub id: i32,
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
        sqlx::query_as!(
            QuoteModel,
            r#"INSERT INTO quotes (ticker, date, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
            self.ticker,
            self.date,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume
        ).fetch_one(db_pool).await
    }
    pub async fn delete(&self, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"DELETE FROM quotes WHERE id = $1 RETURNING *"#,
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn update(&self, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"UPDATE quotes SET ticker = $1, date = $2, open = $3, high = $4, low = $5, close = $6, volume = $7 WHERE id = $8 RETURNING *"#,
            self.ticker,
            self.date,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.id
        ).fetch_one(db_pool).await
    }
    pub async fn get_all(db_pool: &sqlx::PgPool) -> Result<Vec<QuoteModel>, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes"#
        ).fetch_all(db_pool).await
    }
    pub async fn get_by_ticker(ticker: String, db_pool: &sqlx::PgPool) -> Result<Vec<QuoteModel>, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE ticker = $1"#,
            ticker
        ).fetch_all(db_pool).await
    }
    pub async fn get_by_id(id: i32, db_pool: &sqlx::PgPool) -> Result<QuoteModel, sqlx::Error> {
        sqlx::query_as!(
            QuoteModel,
            r#"SELECT * FROM quotes WHERE id = $1"#,
            id
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
}