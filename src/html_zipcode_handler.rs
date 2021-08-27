use actix_web::HttpRequest;
use actix_web::Responder;
use log::info;

pub async fn handle(req: HttpRequest) -> impl Responder {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("html_zipcode_handler {}", zipcode);
    "<html><body><p>Hello World</p></body></html>"
}
