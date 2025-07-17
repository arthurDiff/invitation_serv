use actix_web::{HttpRequest, HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;

use crate::{
    idempotency::{IdempotencyKey, try_get_response},
    response::e500,
};

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct DateRange {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct CreateEventRequest {
    name: String,
    description: Option<String>,
    budget: Option<f64>,
    timeframe: Option<DateRange>,
}

#[tracing::instrument(name = "Creating an event", skip(req, redis, sess, _payload), fields(user_id=sess.sub))]
pub async fn create_event(
    sess: web::ReqData<ClerkJwt>,
    req: HttpRequest,
    redis: web::Data<redis::Client>,
    _payload: web::Payload,
) -> impl Responder {
    if let Some(idem_header) = req.headers().get("idempotency-key") {
        let idem_key = match IdempotencyKey::try_from(idem_header).map_err(e500) {
            Ok(key) => key,
            Err(err_res) => return err_res,
        }
        .attach("create_event", &sess.sub);

        let _ = try_get_response(&redis, &idem_key);
    }

    HttpResponse::Ok().finish()
}
