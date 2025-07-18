use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;
use redis::aio::MultiplexedConnection;

use crate::{
    idempotency::{IdempotencyKey, try_get_response},
    response::{e400, e500},
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

#[tracing::instrument(name = "Creating an event", skip(req, req_data, redis_conn, sess), fields(user_id=sess.sub))]
pub async fn create_event(
    req: HttpRequest,
    req_data: web::Json<CreateEventRequest>,
    sess: web::ReqData<ClerkJwt>,
    redis_conn: web::Data<MultiplexedConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    let idem_key = IdempotencyKey::try_from(req.headers())
        .map_err(e400)?
        .attach("create_event", &sess.sub);

    if let Some(saved_res) = try_get_response(&mut redis_conn.get_ref().clone(), &idem_key).await? {
        return Ok(saved_res);
    }

    // let query = sqlx::query!(
    //     r#"INSERT INTO event (
    //         owner_id,
    //         name,
    //         description,
    //         budget,
    //         timeframe
    //     )
    //     VALUES ($1, $2, $3, $4, $5)
    //     "#,
    //     sess.sub,
    //     req_data.name,
    //     req_data.description,
    //     req_data.budget,
    //     req_data.timeframe
    // );

    Ok(HttpResponse::Ok().finish())
}
