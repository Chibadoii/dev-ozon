use crate::common::config::GlobalConfig;
use actix_web::{web, HttpResponse, Responder};
use crate::application::processing::{info_prices_proc, product_list_proc};

pub async fn health_check() -> impl Responder {
    format!("I'm alive")
}

pub async fn product_list(config: web::Data<GlobalConfig>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(product_list_proc(config).await)
}

// Todo: Нормально описать хендлеры
pub async fn info_prices(config: web::Data<GlobalConfig>) -> impl Responder {
    let _some =info_prices_proc(config).await;
    HttpResponse::Ok().finish()
}
