use actix_web::web::Data;
use sqlx::Row;
use crate::application::API;
use crate::application::common_processing::processing;
use crate::common::config::GlobalConfig;
use crate::presentation::ResponseProduct;

pub async fn product_list_proc(config: Data<GlobalConfig>)-> Vec<ResponseProduct>{
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
    items
}

pub async fn info_prices_proc(_config: Data<GlobalConfig>){
    todo!()
}
