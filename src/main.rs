use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

pub mod application;
mod common;
mod presentation;

use crate::application::handlers::{info_prices, product_list};
use crate::common::config::{DBConfig, GlobalConfig};
use application::handlers::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_server={log_level},actix_web={log_level}",
            log_level = "DEBUG"
        ),
    );
    env_logger::init();
    dbg!("pred_proc");
    let global_config = GlobalConfig::new().await;
    let _insert_migrations = DBConfig::migrations(&global_config.db_config).await;

    //let _ = processing(&global_config).await;

    dbg!("postproc");
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/get_product_list", web::get().to(product_list))
            .route("/get_info_prices", web::get().to(info_prices))
            .app_data(Data::new(global_config.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
