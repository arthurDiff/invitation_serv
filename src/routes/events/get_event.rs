use actix_web::{HttpResponse, Responder, web};
use clerk_rs::validators::authorizer::ClerkJwt;

pub async fn get_event(
    // _evt_pth: web::Path<super::EventPath>,
    _sess: web::ReqData<ClerkJwt>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
