use actix_web::{web, App, HttpServer};
pub mod application;
mod common;
mod presentation;

use crate::application::processing::processing;
use crate::common::config::OzonConfig;
use application::handlers::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dbg!("pred_proc");
    let config = OzonConfig::new();
    let _ = processing(&config).await;
    dbg!("postproc");
    HttpServer::new(|| App::new().route("/", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
