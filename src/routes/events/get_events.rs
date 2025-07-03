use actix_web::{HttpResponse, Responder, web};
use clerk_rs::validators::authorizer::ClerkJwt;

#[tracing::instrument(name = "Getting List of Events associated with user", skip(_sess))]
pub async fn get_events(_sess: web::ReqData<ClerkJwt>) -> impl Responder {
    HttpResponse::Ok().finish()
}
