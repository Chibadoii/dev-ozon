pub mod handlers;
pub mod common_processing;
mod processing;

pub enum API{
    ProductList,
    InfoPrices,
}

impl API{
    fn as_str(&self)-> &'static str{
        match self{
            API::ProductList => "https://api-seller.ozon.ru/v2/product/list",
            API::InfoPrices => "https://api-seller.ozon.ru/v4/product/info/prices"
        }
    }
}
