mod handlers;
mod models;
mod schema;
mod scheduler;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;

pub struct AppState {
    pub db_pool: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address = "0.0.0.0";
    let port = 8080;

    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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
    
    scheduler::start(pool.clone());
    
    println!("🚀 Started server on {}:{}", ip_address, port);
    HttpServer::new(move || {
        App::new()
        .wrap(Cors::permissive())
        .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
        .configure(handlers::configure)
        .wrap(Logger::default())
    })
    .bind((ip_address, port))?
    .run()
    .await
}