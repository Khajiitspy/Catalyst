use actix_web::{HttpRequest, Responder};

pub async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello {}!", req.match_info().get("name").unwrap_or("World"))
}
