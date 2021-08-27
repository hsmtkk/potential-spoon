use actix_web::{HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn handle(req: HttpRequest) -> HttpResponse {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("json_zipcode_handler {}", zipcode);
    HttpResponse::Ok()
    .content_type("application/json")
    .body(r#"{"foo":"bar"}"#)
}
