use crate::application::processing::processing;
use crate::common::config::GlobalConfig;
use crate::presentation::ResponseProduct;
use actix_web::{web, HttpResponse, Responder};
use sqlx::Row;
use crate::application::API;

pub async fn health_check() -> impl Responder {
    format!("I'm alive")
}

pub async fn product_list(config: web::Data<GlobalConfig>) -> impl Responder {
    let data = processing(&config, API::ProductList).await;
    let items: Vec<ResponseProduct> = data
        .iter()
        .map(|row| ResponseProduct {
            product_id: row.get("product_id"),
            offer_id: row.get("offer_id"),
            is_fbo_visible: row.get("is_fbo_visible"),
            is_fbs_visible: row.get("is_fbs_visible"),
            archived: row.get("archived"),
            is_discounted: row.get("is_discounted"),
        })
        .collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(items)
}

// Todo: Нормально описать хендлеры
pub async fn info_prices(config: web::Data<GlobalConfig>) -> impl Responder {
    let data = processing(&config, API::InfoPrices).await;
    let items: Vec<ResponseProduct> = data
        .iter()
        .map(|row| ResponseProduct {
            product_id: row.get("product_id"),
            offer_id: row.get("offer_id"),
            is_fbo_visible: row.get("is_fbo_visible"),
            is_fbs_visible: row.get("is_fbs_visible"),
            archived: row.get("archived"),
            is_discounted: row.get("is_discounted"),
        })
        .collect();
    format!("get info prices");
    HttpResponse::Ok().finish()
}
