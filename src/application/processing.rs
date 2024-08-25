use crate::common::config::OzonConfig;
use reqwest::Client;
use std::fs::File;
use std::io::{stdin, Read};

const PRODUCT_LIST: &str = "https://api-seller.ozon.ru/v2/product/list";
//Модуль посвещается обработчикам api
pub async fn processing(config: &OzonConfig) {
    println!("Выберите номер ozon_api");

    let mut api_version = String::new();
    stdin()
        .read_line(&mut api_version)
        .expect("Ошибка с выбором api");

    dbg!("create request client");
    let client = Client::new();

    dbg!("{}", &api_version);
    match api_version.trim() {
        "1" => {
            let request_message = communication_with_user().await;
            dbg!("1 api pre");
            dbg!("{}", &request_message);
            let response = client
                .post(PRODUCT_LIST)
                .headers(config.headers.clone())
                .body(request_message)
                .send()
                .await
                .expect("response error");
            dbg!("1 api post");
            println!("{:?}", response);
            dbg!("post response");
        }
        "2" => {}
        _ => {
            println!("no api version")
        }
    }
}

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
                let mut get_file = dbg!(File::open(input).expect("Ошибка открытия файла"));

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

pub async fn return_catalog() {}

pub async fn new_request() {}
