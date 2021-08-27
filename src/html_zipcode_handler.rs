use actix_web::{HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn handle(req: HttpRequest) -> HttpResponse {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("html_zipcode_handler {}", zipcode);
    HttpResponse::Ok()
    .content_type("text/html")
    .body("<html><body><p>Hello World</p></body></html>")
}
