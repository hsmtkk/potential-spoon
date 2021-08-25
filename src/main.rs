mod html;
mod index_handler;
mod zipcode_handler;

use iron::Iron;
use log::info;
use router::Router;

fn main() {
    env_logger::init();

    let mut router = Router::new();
    let render = html::Render::new();
    router.get("/", index_handler::IndexHandler::new(), "index");
    router.get(
        "/zipcode",
        zipcode_handler::ZipcodeHandler::new(render),
        "zipcode",
    );

    let addr = get_listen_address();
    let port = get_listen_port();
    let listen_addr = format!("{}:{}", addr, port);
    info!("start {}", listen_addr);

    Iron::new(router).http(listen_addr).unwrap();
}

fn get_listen_address() -> String {
    match std::env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_e) => "127.0.0.1".to_string(),
    }
}

fn get_listen_port() -> i32 {
    match std::env::var("LISTEN_PORT") {
        Ok(port_str) => {
            match port_str.parse::<i32>() {
                Ok(port) => port,
                Err(_e) => 8000,
            }},
        Err(_e) => 8000,
    }
}