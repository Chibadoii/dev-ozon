use crate::application::API;
use crate::common::config::{DBConfig, GlobalConfig, OzonConfig};
use crate::presentation::ResWrapper;
use reqwest::Client;
use sqlx::postgres::PgRow;
use std::fs::File;
use std::io::{stdin, Read};

// todo: из хендлера передается название API, Название Таблицы, Название структуры
//todo: выбор способа ввода - обращение к озон - сохранение в бд - вывоб в ручке актикса

//Модуль посвещается обработчикам api
pub async fn processing(config: &GlobalConfig, api: API) -> Vec<PgRow> {
    // Получение от пользователя фильтров запроса
    let request_message = communication_with_user().await;

    // Получение запрашиваемой структуры с озона
    let response = ozon_request(&config.ozon_config, request_message, api).await;

    // Охранение ответов озон
    let _err_storage = storage_response(response, &config.db_config).await;

    get_all_items(&config.db_config).await
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

/// Посылает запрос озону и принимает ответ
pub async fn ozon_request(config: &OzonConfig, request_message: String, api: API) -> ResWrapper {
    let response = Client::new()
        .post(api.as_str())
        .headers(config.headers.clone())
        .body(request_message)
        .send()
        .await
        .expect("response error");

    let response: ResWrapper = response.json().await.unwrap();
    dbg!("get response");
    response
}

/// Сохраняет api response
pub async fn storage_response(response: ResWrapper, db_config: &DBConfig) -> sqlx::Result<()> {
    dbg!("save_in_storage");

//todo добавить diesel, вариант info prices слишком долго описывать
    sqlx::query("INSERT INTO common_info_product (total, last_id) VALUES ($1, $2)")
        .bind(response.result.total as i32)
        .bind(&response.result.last_id)
        .fetch_all(&db_config.db_connection)
        .await
        .unwrap();

    //todo не проходит
    dbg!("{}", &response);
    dbg!("save first part");
    for (count, item) in response.result.items.iter().enumerate() {
        sqlx::query("INSERT INTO items (id, product_id, offer_id, is_fbo_visible, is_fbs_visible, archived, is_discounted) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING")
                .bind(count as i32)
                .bind(item.product_id)
                .bind(&item.offer_id)
                .bind(&item.is_fbo_visible)
                .bind(&item.is_fbs_visible)
                .bind(&item.archived)
                .bind(&item.is_discounted)
                .fetch_all(&db_config.db_connection)
                .await
                .unwrap();
    }
    dbg!("db_response");
    Ok(())
}

pub async fn get_all_items(db_config: &DBConfig) -> Vec<PgRow> {
    sqlx::query("select * from items")
        .fetch_all(&db_config.db_connection)
        .await
        .unwrap()
}

pub async fn custom_request(pool: &DBConfig) {

    println!("Для ввода кастомного запроса введите \" input \" ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("err read user input");

    match input.trim() {
        "input" => { println!("Введите команду");
            stdin().read_line(&mut input).expect("err read user input");
                sqlx::query(&input)
                    .fetch_all(&pool.db_connection)
                    .await
                    .expect("err get response from db");
        },

        _ => println!("Что то добавить")
    }
}
