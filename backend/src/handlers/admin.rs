use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use crate::{
    models,
    AppState,
};

#[delete("/nuke")]
pub async fn nuke_databse(state: web::Data<AppState>) -> impl Responder {
    let result = models::nuke_database(&state.db_pool).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Database nuked"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to nuke database")
    }
}

pub fn configure_admin(config: &mut web::ServiceConfig) {
    config.service(nuke_databse);
}
