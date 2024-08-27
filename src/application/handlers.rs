use actix_web::{HttpResponse, Responder};
pub async fn health_check() -> impl Responder {
    format!("I'm alive")
}

pub async fn product_list() -> impl Responder {
    format!("get product list");
    HttpResponse::Ok().finish()
}

// Todo: Нормально описать хендлеры
pub async fn info_prices() -> impl Responder {
    format!("get info prices");
    HttpResponse::Ok().finish()
}
