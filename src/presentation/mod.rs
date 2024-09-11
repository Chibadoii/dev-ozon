pub mod info_prices_dto;
pub mod product_list_dto;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestProduct {
    #[serde(rename = "responseItems")]
    pub filter: Filter,
    pub last_id: String,
    pub limit: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub offer_id: String,
    pub product_id: String,
    pub visibility: String,
}
