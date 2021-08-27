use actix_web::HttpRequest;
use actix_web::Responder;
use log::{info};

pub async fn handle(req: HttpRequest) -> impl Responder {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("json_zipcode_handler {}", zipcode);
    r#"{"foo":"bar"}"#
}
