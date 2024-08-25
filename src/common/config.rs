use actix_web::http::header;
use actix_web::http::header::HeaderName;
use std::str::FromStr;
use http::header::{HeaderMap, HeaderValue};
pub struct GlobalConfig {
    ozon_config: OzonConfig,
    actix_config: ActixWebConfig,
}

// Заглушка
pub struct ActixWebConfig {
    pub moc: Option<String>,
}

pub struct OzonConfig {
    pub headers: HeaderMap,
}

impl ActixWebConfig {
    pub fn new() -> Self {
        todo!()
    }
}

impl OzonConfig {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Client-id", HeaderValue::try_from(dotenv::var("CLIENT_ID_NUMBER").expect("Err set client id"))
                .expect("Err read client id"),
        );
        headers.insert(
            "Api-Key", HeaderValue::try_from(dotenv::var("CLIENT_API_KEY").expect("Err set apikey"))
                .expect("Err read client apikey"),
        );
        headers.insert(
           "Content_Type", HeaderValue::from_static("application/json"),
        );
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        Self { headers }
    }
}
