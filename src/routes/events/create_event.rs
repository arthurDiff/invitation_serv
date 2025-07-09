use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;

#[derive(serde::Deserialize, Debug)]
pub struct DateRange {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateEventRequest {
    name: String,
    description: Option<String>,
    budget: Option<f64>,
    timeframe: Option<DateRange>,
}

#[tracing::instrument(name = "Creating an event", skip(sess, payload))]
pub async fn create_event(sess: web::ReqData<ClerkJwt>, mut payload: web::Payload) -> impl Responder {
    // let result = body.await;
    // println!("{:?}", result);
    HttpResponse::Ok().finish()
}
