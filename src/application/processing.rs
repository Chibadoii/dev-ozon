use crate::common::config::{DBConfig, GlobalConfig, OzonConfig};
use crate::presentation::ResWrapper;
use http::StatusCode;
use reqwest::{Client, Error, Response};
use rusqlite::params;
use std::fs::File;
use std::io::{stdin, Read};
use serde::{Deserialize, Serialize};

const PRODUCT_LIST: &str = "https://api-seller.ozon.ru/v2/product/list";
// todo: из хендлера передается название API, Название Таблицы, Название структуры
//todo: выбор способа ввода - обращение к озон - сохранение в бд - вывоб в ручке актикса

//Модуль посвещается обработчикам api
pub async fn processing(config: &GlobalConfig) {

    // Получение от пользователя фильтров запроса
    let request_message = communication_with_user().await;

    // Получение запрашиваемой структуры с озона
    let response = ozon_request(&config.ozon_config, request_message).await;

    //println!("{:?}", &response);
    // Охранение ответов озон
    storage_response(response, &config.db_config);
}

// Возвращает сообщение содержащее запрос для Ozon
pub async fn communication_with_user() -> String {
    loop {
        println!("Введите 1 для записи параметоров запроса");
        println!("Введите 2, что бы указать путь к файлу с параметрам запроса");

        let mut input_method = String::new();
        stdin()
            .read_line(&mut input_method)
            .expect("Err proc user event method");

        match input_method.trim() {
            "1" => {
                println!("Введите параметры запроса");
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("Err read user api reqwest");
                return input;
            }
            "2" => {
                println!("Введите путь к файлу");
                let mut input = String::new();

                stdin()
                    .read_line(&mut input)
                    .expect("Некорректный путь к файлу");

                //let path = std::path::Path::new("./test_request/req.json");
                let mut get_file = dbg!(File::open(input.trim())).expect("Ошибка открытия файла");

                let mut request = String::new();
                get_file
                    .read_to_string(&mut request)
                    .expect("Ошибка считывания файла");

                return request;
            }
            _ => {
                println!("Недопустимый вариант, попробуем заново");
            }
        }
    }
}

pub async fn ozon_request(config: &OzonConfig, request_message: String) -> ResWrapper {
    dbg!("create request client");
    let response = Client::new()
        .post(PRODUCT_LIST)
        .headers(config.headers.clone())
        .body(request_message)
        .send()
        .await
        .expect("response error");

    let response: ResWrapper = response.json().await.unwrap();

    response
}

pub async fn storage_response(response: ResWrapper, db_config: &DBConfig) -> rusqlite::Result<()> {
    db_config.db_connection.execute(
        "INSERT INTO common_info_product (total, last_id) VALUES (?1, ?2)",
        params![response.result.total, response.result.last_id],
    )?;

    let _ = response.result.items.iter().map(|item| {
        db_config.db_connection
            .execute("INSERT INTO items (product_id, offer_id, is_fbo_visible, is_fbs_visible, archived, is_discounted) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                     params![item.product_id, item.offer_id, item.is_fbo_visible, item.is_fbs_visible, item.archived, item.is_discounted])
    });

    let mut db_response = db_config.db_connection.prepare("select * from items")?;
    let test_iter = db_response.query_map([], |row| {
        Ok(TestResponse{
            product_id: row.get(0)?,
            id: row.get(1)?
        })
    })?;

    for item in test_iter{
        match item{
            Ok(item) => println!("{:?}", item),
            Err(e) => eprintln!("err: {}", e)
        }
    }

    println!("{:?}", db_response);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse{
    pub product_id: i64,
    pub id: i64,
}
