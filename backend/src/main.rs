use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Deserialize;

#[derive(Deserialize)]
struct FinancialInfo {
    start: u64,
    returns: f32,
    fire_rate: f64,
    expenses: u64,
    monthly_investment: u64
}
enum SimulationType {
    MarkovChain,
    LSTM
}
#[derive(Deserialize)]
struct SimulationInfo {
    financials: FinancialInfo,
    simulations: u8
    
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address = "127.0.0.1";
    let port = 8080;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Server listening on {}:{}", ip_address, port);
    HttpServer::new(|| {
        App::new()
        .service(fire_calculator)
        .wrap(Logger::default())
    })
    .bind((ip_address, port))?
    .run()
    .await
}