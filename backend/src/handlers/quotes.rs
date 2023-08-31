use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{
    AppState,
    models::quotes::QuoteModel, schema::quotes::QuoteJson,
    schema::Pagination,
    models::stocks::is_valid_ticker
};

#[get("/quotes")]
pub async fn get_all_quotes(page: web::Query<Pagination>, state: web::Data<AppState>) -> impl Responder {
    let result = QuoteModel::get_all_paginated(page.into_inner(), &state.db_pool).await;
    match result {
        Ok(quotes) => HttpResponse::Ok().json(quotes.into_iter().map(|quote| quote.into()).collect::<Vec<QuoteJson>>()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get quotes")
    }
}

#[post("/quotes")]
pub async fn add_ticker(state: web::Data<AppState>, ticker: web::Json<String>) -> impl Responder {
    let valid = is_valid_ticker(&ticker.clone()).await;
    if !valid {
        return HttpResponse::BadRequest().body("Invalid ticker");
    }
    let result = QuoteModel::populate_ticker(ticker.into_inner(), &state.db_pool).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Ticker added successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add ticker")
    }
}

pub fn configure_quotes(config: &mut web::ServiceConfig) {
    config.service(get_all_quotes);
    config.service(add_ticker);
}

