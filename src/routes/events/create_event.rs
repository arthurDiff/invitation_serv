use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{DateTime, Utc};
use clerk_rs::validators::authorizer::ClerkJwt;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::{
    async_ext::AsyncExt,
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

    let mut tx = pool
        .begin()
        .await
        .map_err_async(|e| async {
            _ = rollback_precache(&mut redis_conn.clone(), &idem_key).await;
            e500(e)
        })
        .await?;

    let new_evt = sqlx::query_as!(
        Event,
        r#"INSERT INTO event (
            name,
            description,
            budget,
            starts_at,
            ends_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
        "#,
        req_data.name,
        req_data.description,
        req_data.budget,
        req_data.starts_at,
        req_data.ends_at
    )
    .fetch_one(&mut *tx)
    .await
    .map_err_async(|e| async {
        _ = rollback_precache(&mut redis_conn.clone(), &idem_key).await;
        e500(e)
    })
    .await?;

    if let Err(member_err) = sqlx::query!(
        r#"INSERT INTO member (user_id,event_id,role)
        VALUES ($1, $2, 'owner'::member_role);"#,
        sess.sub,
        new_evt.id,
    )
    .execute(&mut *tx)
    .await
    .map_err(e500)
    {
        return Err(member_err);
    };

    let res = HttpResponse::Ok().json(ResponseBody::<Event> {
        message: "event created".into(),
        data: Some(new_evt),
    });

    save_response(&mut redis_conn, &idem_key, res).await
}
