mod handlers;
mod models;
mod schema;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use serde::Deserialize;
use serde::Serialize;
use yahoo_finance_api as yahoo;
use chrono::{NaiveDate, Utc, TimeZone};
use std::time::Duration;
use strum_macros::{EnumString, Display};
use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;
use actix_rt::{spawn, time::interval};
pub struct AppState {
    pub db_pool: PgPool
}
#[derive(Deserialize)]
struct FinancialInfo {
    start: u64,
    returns: f32,
    fire_rate: f64,
    expenses: u64,
    monthly_investment: u64
}
#[derive(Deserialize)]
struct SimulationInfo {
    financials: FinancialInfo,
    num_simulations: u8
    
}
#[derive(Deserialize)]
struct Ticker {
    symbol: String,
    interval: Option<Interval>,
    range: Option<Range>
}
#[derive(Deserialize, Serialize, EnumString, Display, Clone, Copy)]
enum Interval {
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    Daily,
    #[serde(rename = "1wk")]
    #[strum(serialize = "1wk")]
    Weekly,
    #[serde(rename = "1mo")]
    #[strum(serialize = "1mo")]
    Monthly
}
#[derive(Deserialize, Serialize, EnumString, Display, Clone, Copy)]
enum Range {
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    Daily,
    #[serde(rename = "5d")]
    #[strum(serialize = "5d")]
    FiveDays,
    #[serde(rename = "1mo")]
    #[strum(serialize = "1mo")]
    Monthly,
    #[serde(rename = "3mo")]
    #[strum(serialize = "3mo")]
    ThreeMonths,
    #[serde(rename = "6mo")]
    #[strum(serialize = "6mo")]
    SixMonths,
    #[serde(rename = "1y")]
    #[strum(serialize = "1y")]
    Yearly,
    #[serde(rename = "2y")]
    #[strum(serialize = "2y")]
    TwoYears,
    #[serde(rename = "5y")]
    #[strum(serialize = "5y")]
    FiveYears,
    #[serde(rename = "10y")]
    #[strum(serialize = "10y")]
    TenYears,
    #[serde(rename = "ytd")]
    #[strum(serialize = "ytd")]
    YearToDate,
    #[serde(rename = "max")]
    #[strum(serialize = "max")]
    Max
}

#[derive(Serialize, Deserialize)]
enum TickerResult {
    #[serde(rename = "quotes")]
    Quotes(Vec<QuoteResult>),
    #[serde(rename = "error")]
    Error(String)
}
#[derive(Serialize, Deserialize)]
struct QuoteResult {
    timestamp: NaiveDate,
    open: f64,
    close: f64,
    volume: u64
}
#[post("/fire")]
async fn fire_calculator(info: web::Json<FinancialInfo>) -> impl Responder {
    if (info.fire_rate < 0.0) || (info.fire_rate > 1.0) {
        return HttpResponse::BadRequest().body("Fire rate must be between 0 and 1");
    }
    if info.returns < 0.0 {
        return HttpResponse::BadRequest().body("Returns must not be negative");
    }
    let mut current_value = info.start as f64;
    let mut years = 0;
    while (current_value * info.fire_rate) < info.expenses as f64 {
        current_value = current_value * (1.0 + info.returns as f64) + info.monthly_investment as f64;
        years += 1;
    }
    HttpResponse::Ok().body(format!("You can retire in {} years", years))
}

#[get("/ticker")]
async fn ticker(info: web::Query<Ticker>) -> Result<impl Responder> {
    let interval = match info.interval {
        Some(interval) => interval,
        None => Interval::Daily
    };
    let range = match info.range {
        Some(range) => range,
        None => Range::Max
    };
    let provider = yahoo::YahooConnector::new();
    // returns historic quotes with daily interval
    let resp = provider.get_quote_range(&info.symbol, interval.to_string().as_str(), range.to_string().as_str()).await;
    let quotes;
    match resp {
        Ok(resp) => {
            quotes = resp.quotes().unwrap_or_default();
        },
        Err(e) => {
            println!("Error: {}", e);
            return Ok(web::Json(TickerResult::Error(e.to_string())));
        }
    }
    let quotes_results = quotes.into_iter().map(|quote| {
        QuoteResult {
            timestamp: Utc.timestamp_opt(quote.timestamp as i64, 0).unwrap().date_naive(),
            open: quote.open,
            close: quote.close,
            volume: quote.volume
        }
    }).collect::<Vec<_>>();
    Ok(web::Json(TickerResult::Quotes(quotes_results)))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address = "127.0.0.1";
    let port = 8080;

    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            println!("🔥🔥🔥");
        }
    });
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };


    println!("🚀 Started server on {}:{}", ip_address, port);
    HttpServer::new(move || {
        App::new()
        .wrap(Cors::permissive())
        .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
        .configure(handlers::configure)
        .service(fire_calculator)
        .service(ticker)
        .wrap(Logger::default())
    })
    .bind((ip_address, port))?
    .run()
    .await
}