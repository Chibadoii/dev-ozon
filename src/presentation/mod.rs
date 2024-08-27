use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestProduct {
    #[serde(rename = "responseItems")]
    pub filter: Filter,
    pub last_id: String,
    pub limit: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    offer_id: String,
    product_id: String,
    visibility: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResWrapper {
    result: Res,
}
#[derive(Serialize, Deserialize, Debug)]
struct Res {
    items: Vec<ResponseProduct>,
    total: i32,
    last_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseProduct {
    product_id: i64,
    offer_id: String,
    is_fbo_visible: bool,
    is_fbs_visible: bool,
    archived: bool,
    is_discounted: bool,
    // status:bool
}
