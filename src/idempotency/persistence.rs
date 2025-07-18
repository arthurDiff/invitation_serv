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
    pub headers: Vec<(String, String)>,
    pub body: JsonValue,
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

    let mut response = HttpResponse::build(StatusCode::from_u16(saved_res.status_code).map_err(e500)?);

    saved_res.headers.into_iter().for_each(|(k, v)| {
        response.insert_header((k, v));
    });

    Ok(Some(response.json(saved_res.body)))
}
