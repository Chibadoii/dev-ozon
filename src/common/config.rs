use http::header::{HeaderMap, HeaderValue};
use rusqlite::Connection;

const DBADRESS: &str = "example";
pub struct GlobalConfig {
    ozon_config: OzonConfig,
    actix_config: ActixWebConfig,
    db_config: DBConfig,
}

// Заглушка
pub struct ActixWebConfig {
    pub moc: Option<String>,
}

pub struct DBConfig{
    db_pool: Connection,
}

impl DBConfig{
    fn new () -> Self{
        let connection = Connection::open(DBADRESS);
        Self{
            db_pool: connection.expect("err connection")
        }
    }
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
            "Client-id",
            HeaderValue::try_from(dotenv::var("CLIENT_ID_NUMBER").expect("Err set client id"))
                .expect("Err read client id"),
        );
        headers.insert(
            "Api-Key",
            HeaderValue::try_from(dotenv::var("CLIENT_API_KEY").expect("Err set apikey"))
                .expect("Err read client apikey"),
        );
        headers.insert("Content_Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        Self { headers }
    }
}
