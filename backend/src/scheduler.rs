use actix_rt::{spawn, time::interval};
use std::time::Duration;
use crate::models::quotes::QuoteModel;
use sqlx::postgres::PgPool;
pub fn start(db_pool: PgPool) {
    start_quote_updater(db_pool.clone());
}

fn start_quote_updater(db_pool: PgPool) {
    spawn(async move {
        let mut interval = interval(Duration::from_secs(60*60*24));
        loop {
            interval.tick().await;
            println!("🚀 Updating quotes...");
            let result = QuoteModel::update_quotes(&db_pool).await;
            match result {
                Ok(_) => println!("✅ Quotes updated successfully!"),
                Err(err) => println!("🔥 Failed to update quotes: {:?}", err)
            }
        }
    });
}