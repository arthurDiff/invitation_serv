use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;

use crate::request::RequestDeserializer;

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

#[tracing::instrument(name = "Creating an event", skip(payload))]
pub async fn create_event(_: web::ReqData<ClerkJwt>, payload: web::Payload) -> impl Responder {
    let _req_body = payload.deserialize_as::<CreateEventRequest>().await;
    // println!("{:?}", result);
    HttpResponse::Ok().finish()
}
