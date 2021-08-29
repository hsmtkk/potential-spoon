use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Data;
use crate::zipcode_api::Searcher;
use log::info;

pub async fn handle(req: HttpRequest, searcher:Data<Searcher>) -> HttpResponse {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("json_zipcode_handler {}", zipcode);
    let json = searcher.search(zipcode).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(json)
}
