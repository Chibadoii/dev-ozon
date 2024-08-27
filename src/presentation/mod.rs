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

#[derive(Serialize, Deserialize, Debug)]
pub struct ResWrapper {
    pub result: CommonInfoProduct,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonInfoProduct {
    pub items: Vec<ResponseProduct>,
    pub total: i32,
    pub last_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseProduct {
    pub product_id: i64,
    pub offer_id: String,
    pub is_fbo_visible: bool,
    pub is_fbs_visible: bool,
    pub archived: bool,
    pub is_discounted: bool,
    // status:bool
}
