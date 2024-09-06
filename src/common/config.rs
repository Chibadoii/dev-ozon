use http::header::{HeaderMap, HeaderValue};
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub ozon_config: OzonConfig,
    pub actix_config: ActixWebConfig,
    pub db_config: DBConfig,
}

impl GlobalConfig {
    pub async fn new() -> Self {
        Self {
            ozon_config: OzonConfig::new(),
            actix_config: ActixWebConfig::new(),
            db_config: DBConfig::new().await,
        }
    }
}

/// Заглушка
#[derive(Debug, Clone)]
pub struct ActixWebConfig {
    pub moc: Option<String>,
}

impl ActixWebConfig {
    pub fn new() -> Self {
        Self { moc: None }
    }
}

/// Инициализация подключения и выполнение первичных миграций
#[derive(Debug, Clone)]
pub struct DBConfig {
    pub db_connection: Pool<Postgres>,
}

impl DBConfig {
    pub async fn new() -> Self {
        Self {
            db_connection: PgPoolOptions::new()
                .max_connections(10)
                .connect(env::var("DB_NAME").expect("Err get db name").as_str())
                .await
                .expect("Err create connection"),
        }
    }

    pub async fn migrations(&self) -> Result<(), Error> {
       // let migration_path = ["drop", "create"];

       // for path in migration_path.iter(){
       //      let migrations_dir = std::path::Path::new(&format!("migrations/{}", path));
        let migrations_dir = std::path::Path::new("migrations");
        if migrations_dir.is_dir(){
            for entry in fs::read_dir(&migrations_dir).expect("err read dir") {
                let file_name = entry.unwrap().file_name();
                let migrations_path = migrations_dir.join(file_name.clone());
                let migration_script = fs::read_to_string(&migrations_path).expect(&format!(
                    "Err read migrations: {}",
                    migrations_path.display()
                ));

                let rows = sqlx::query(&*migration_script)
                    .execute(&self.db_connection)
                    .await
                    .expect("err insert migrations");
                println!("{:?}", rows);
                println!("{}", migration_script)
            }
        }


       // }

        Ok(())
    }
}

/// Конфиг для общения с api ozon
#[derive(Debug, Clone)]
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
