pub mod stocks;
pub mod quotes;
pub mod trades;

use sqlx::postgres::PgPool;

pub async fn nuke_database(db_pool: &PgPool) -> Result<(), sqlx::Error> {
    stocks::StockModel::delete_all(db_pool).await?;
    quotes::QuoteModel::delete_all(db_pool).await?;
    trades::TradeModel::delete_all(db_pool).await?;
    Ok(())
}