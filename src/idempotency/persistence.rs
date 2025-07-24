use actix_web::{HttpResponse, http::StatusCode};
use redis::{AsyncTypedCommands, aio::MultiplexedConnection};
use serde_json::Value as JsonValue;

use crate::{
    idempotency::IdempotencyKey,
    response::{e409, e500},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SavedResponse {
    pub status_code: u16,
    pub headers: Vec<(String, Vec<u8>)>,
    pub body: JsonValue,
}

impl SavedResponse {
    fn to_http_response(self) -> Result<HttpResponse, actix_web::Error> {
        let mut response = HttpResponse::build(StatusCode::from_u16(self.status_code).map_err(e500)?);

        self.headers.into_iter().for_each(|(k, v)| {
            response.insert_header((k, v));
        });

        Ok(response.json(self.body))
    }
}

pub enum IdempotencyError {
    Conflict(String),
    Redis(redis::RedisError),
    Unknown(anyhow::Error),
}

const PLACEHOLDER_IDEM_VALUE: &str = "nil";
pub async fn try_get_response(
    redis_conn: &mut MultiplexedConnection,
    key: &IdempotencyKey,
) -> Result<Option<HttpResponse>, actix_web::Error> {
    let Some(saved_res) = redis_conn.get(&key.0).await.map_err(e500)? else {
        // 60 * 15 | 15m
        let _: () = redis_conn
            .set_ex(&key.0, PLACEHOLDER_IDEM_VALUE, 900)
            .await
            .map_err(e500)?;
        return Ok(None);
    };

    if saved_res == PLACEHOLDER_IDEM_VALUE {
        return Err(e409("Operation already in progress"));
    }

    let saved_res = serde_json::from_str::<SavedResponse>(&saved_res).map_err(e500)?;

    Ok(Some(saved_res.to_http_response()?))
}

pub async fn save_response(
    redis_conn: &mut MultiplexedConnection,
    key: &IdempotencyKey,
    status_code: StatusCode,
    headers: Vec<(String, Vec<u8>)>,
    body: impl serde::Serialize,
) -> Result<HttpResponse, actix_web::Error> {
    let s_res = SavedResponse {
        status_code: status_code.as_u16(),
        headers,
        body: serde_json::to_value(body).map_err(e500)?,
    };

    if let Err(err) = redis_conn
        .set_ex(&key.0, serde_json::to_string(&s_res).map_err(e500)?, 900)
        .await
        .map_err(e500)
    {
        _ = rollback_precache(redis_conn, key).await?;
        return Err(err);
    }

    s_res.to_http_response()
}

pub async fn rollback_precache(
    redis_conn: &mut MultiplexedConnection,
    key: &IdempotencyKey,
) -> Result<(), actix_web::Error> {
    _ = redis_conn.del(&key.0).await.map_err(e500)?;
    Ok(())
}
