use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub result: Result,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result{
    pub items: Item,
    pub total: i32,
    pub last_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item{
    pub product_id: i32,
    pub offer_id: String,
    pub price: Price,
    pub price_index: String,
    pub commissions: Commissions,
    pub marketing_actions: Option<String>,
    pub volume_weight: f64,
    pub price_indexes: PriceIndexes,
    pub acquiring: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price{
    pub price: String,
    pub old_price: String,
    pub premium_price: String,
    pub recommended_price: String,
    pub retail_price: String,
    pub vat: String,
    pub min_ozon_price: String,
    pub marketing_price: String,
    pub marketing_seller_price: String,
    pub min_price: String,
    pub currency_code: String,
    pub auto_action_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commissions{
    pub sales_percent: i32,
    pub fbo_fulfillment_amount: i32,
    pub fbo_direct_flow_trans_min_amount: i32,
    pub fbo_direct_flow_trans_max_amount: i32,
    pub fbo_deliv_to_customer_amount: f64,
    pub fbo_return_flow_amount: i32,
    pub fbo_return_flow_trans_min_amount: i32,
    pub fbo_return_flow_trans_max_amount: i32,
    pub fbs_first_mile_min_amount: i32,
    pub fbs_first_mile_max_amount: i32,
    pub fbs_direct_flow_trans_min_amount: i32,
    pub fbs_direct_flow_trans_max_amount: i32,
    pub fbs_deliv_to_customer_amount: f64,
    pub fbs_return_flow_amount: i32,
    pub fbs_return_flow_trans_min_amount: i32,
    pub fbs_return_flow_trans_max_amount: i32,
    pub sales_percent_fbo: i32,
    pub sales_percent_fbs: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceIndexes{
    pub price_indexes: String,
    pub external_index_data: IndexData,
    pub ozon_index_data: IndexData,
    pub self_marketplaces_index_data: IndexData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexData{
    pub minimal_price: String,
    pub minimal_price_currency: String,
    pub price_index_value: i32
}
