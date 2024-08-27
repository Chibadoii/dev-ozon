use http::header::{HeaderMap, HeaderValue};
use rusqlite::Connection;
use std::{env, fs};

pub struct GlobalConfig {
    pub ozon_config: OzonConfig,
    pub actix_config: ActixWebConfig,
    pub db_config: DBConfig,
}

impl GlobalConfig {
    pub fn new() -> Self {
        Self {
            ozon_config: OzonConfig::new(),
            actix_config: ActixWebConfig::new(),
            db_config: DBConfig::new(),
        }
    }
}

/// Заглушка
pub struct ActixWebConfig {
    pub moc: Option<String>,
}

impl ActixWebConfig {
    pub fn new() -> Self {
        Self { moc: None }
    }
}

/// Инициализация подключения и выполнение первичных миграций
pub struct DBConfig {
    pub db_connection: Connection,
}

impl DBConfig {
    pub fn new() -> Self {
        Self {
            db_connection: Connection::open(env::var("DB_NAME").expect("Err get db name"))
                .expect("Err create connection"),
        }
    }

    pub fn migrations(&self) -> rusqlite::Result<()> {
        let migrations_dir = std::path::Path::new("migrations");
        if let Ok(dir) = fs::read_dir(&migrations_dir) {
            for entry in dir {
                let file_name = entry.unwrap().file_name();
                let migrations_path = migrations_dir.join(file_name.clone());
                let migration_script = fs::read_to_string(&migrations_path).expect(&format!(
                    "Err read migrations: {}",
                    migrations_path.display()
                ));

                self.db_connection.execute(&migration_script, [])?;
            }
        } else {
            panic!("Директории migrations не существует")
        }
        Ok(())
    }
}

/// Конфиг для общения с api ozon
pub struct OzonConfig {
    pub headers: HeaderMap,
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
