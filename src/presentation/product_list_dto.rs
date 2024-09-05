use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResWrapper {
    pub result: crate::presentation::CommonInfoProduct,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonInfoProduct {
    pub items: Vec<crate::presentation::ResponseProduct>,
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
