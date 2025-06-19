use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    // Maybe add version number? maybe not considering this server is internal only
    HttpResponse::Ok()
}
