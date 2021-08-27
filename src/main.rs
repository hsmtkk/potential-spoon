/*
mod html;
mod index_handler;
mod zipcode_api;
mod zipcode_handler;
*/

mod html_zipcode_handler;
mod json_zipcode_handler;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let addr = get_listen_address();
    let port = get_listen_port();
    HttpServer::new(|| {
        App::new()
            .route("/html/{zipcode}", web::get().to(html_zipcode_handler::handle))
            .route("/json/{zipcode}", web::get().to(json_zipcode_handler::handle))
    })
    .bind((addr, port))?
    .run()
    .await
}

fn get_listen_address() -> String {
    match std::env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_e) => "127.0.0.1".to_string(),
    }
}

fn get_listen_port() -> u16 {
    match std::env::var("LISTEN_PORT") {
        Ok(port_str) => {
            match port_str.parse::<u16>() {
                Ok(port) => port,
                Err(_e) => 8000,
            }},
        Err(_e) => 8000,
    }
}