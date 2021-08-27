use actix_web::{HttpRequest, HttpResponse, Responder};
use log::info;
use tera::{Context, Tera};

pub async fn handle(req: HttpRequest) -> HttpResponse {
    let zipcode = req.match_info().get("zipcode").unwrap();
    info!("html_zipcode_handler {}", zipcode);
    let tera = Tera::new("template/*.html").unwrap();
    let mut context = Context::new();
    context.insert("zipcode", &zipcode);
    let html = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok()
    .content_type("text/html")
    .body(html)
}
