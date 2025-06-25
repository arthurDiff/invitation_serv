use actix_web::{HttpResponse, Responder, web};
use clerk_rs::validators::authorizer::ClerkJwt;

pub async fn protected_path(session: web::ReqData<ClerkJwt>) -> impl Responder {
    HttpResponse::Ok().json(session.into_inner())
}
