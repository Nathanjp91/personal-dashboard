use actix_web::{get, web, HttpResponse, Responder};
use crate::{
    AppState,
    models::quotes::QuoteModel, schema::quotes::QuoteJson,
    schema::Pagination
};

#[get("/quotes")]
pub async fn get_all_quotes(page: web::Query<Pagination>, state: web::Data<AppState>) -> impl Responder {
    let result = QuoteModel::get_all_paginated(page.into_inner(), &state.db_pool).await;
    match result {
        Ok(quotes) => HttpResponse::Ok().json(quotes.into_iter().map(|quote| quote.into()).collect::<Vec<QuoteJson>>()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get quotes")
    }
}

pub fn configure_quotes(config: &mut web::ServiceConfig) {
    config.service(get_all_quotes);
}

