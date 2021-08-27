use actix_web::{HttpRequest, HttpResponse};
use crate::zipcode_api::Searcher;
use log::info;

pub async fn handle(req: HttpRequest) -> HttpResponse {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("json_zipcode_handler {}", zipcode);
    let searcher = Searcher::new();
    let json = searcher.search(zipcode).unwrap();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(json)
}
