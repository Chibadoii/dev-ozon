use actix_web::{web, App, HttpServer};
pub mod application;
mod common;
mod presentation;

use crate::application::handlers::{info_prices, product_list};
use crate::application::processing::processing;
use crate::common::config::{DBConfig, GlobalConfig};
use application::handlers::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dbg!("pred_proc");
    let global_config = GlobalConfig::new();
    let _create_db = DBConfig::migrations(&global_config.db_config);

    let _ = processing(&global_config).await;
    dbg!("postproc");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/get_product_list", web::get().to(product_list))
            .route("/get_info_prices", web::get().to(info_prices))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
