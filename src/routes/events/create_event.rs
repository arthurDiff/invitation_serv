use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    idempotency::{IdempotencyKey, rollback_precache, save_response, try_get_response},
    models::Event,
    response::{ResponseBody, e400, e500},
};

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct CreateEventRequest {
    name: String,
    description: Option<String>,
    budget: Option<f32>,
    #[serde(rename = "startsAt")]
    starts_at: Option<DateTime<Utc>>,
    #[serde(rename = "endsAt")]
    ends_at: Option<DateTime<Utc>>,
}

#[tracing::instrument(name = "Creating an event", skip(req, req_data, redis, sess, pool), fields(user_id=sess.sub))]
pub async fn create_event(
    req: HttpRequest,
    req_data: web::Json<CreateEventRequest>,
    sess: web::ReqData<ClerkJwt>,
    redis: web::Data<MultiplexedConnection>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let idem_key = IdempotencyKey::try_from(req.headers())
        .map_err(e400)?
        .attach("create_event", &sess.sub);

    let mut redis_conn = redis.get_ref().clone();
    if let Some(saved_res) = try_get_response(&mut redis_conn, &idem_key).await? {
        return Ok(saved_res);
    }

    let new_evt = match sqlx::query!(
        r#"INSERT INTO event (
            owner_id,
            name,
            description,
            budget,
            starts_at,
            ends_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, created_at;
        "#,
        sess.sub,
        req_data.name,
        req_data.description,
        req_data.budget,
        req_data.starts_at,
        req_data.ends_at
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(e500)
    {
        Ok(new_e) => new_e,
        Err(err) => {
            _ = rollback_precache(&mut redis_conn, &idem_key).await?;
            return Err(err);
        }
    };

    let res_body = ResponseBody::<Event> {
        message: "event created".into(),
        data: Some(Event {
            id: new_evt.id,
            user_id: sess.sub.clone(),
            name: req_data.name.clone(),
            description: req_data.description.clone(),
            budget: req_data.budget,
            starts_at: req_data.starts_at,
            ends_at: req_data.ends_at,
            created_at: new_evt.created_at,
            updated_at: None,
        }),
    };

    _ = save_response(&mut redis_conn, &idem_key, &res_body).await?;

    Ok(HttpResponse::Ok().json(res_body))
}
