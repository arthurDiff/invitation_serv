use actix_web::{HttpResponse, body::to_bytes, http::StatusCode, web::Bytes};
use redis::{AsyncTypedCommands, aio::MultiplexedConnection};

use crate::{
    idempotency::IdempotencyKey,
    response::{e409, e500},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SavedResponseBody {
    #[serde(flatten)]
    values: serde_json::Map<String, serde_json::Value>,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SavedResponse {
    pub status_code: u16,
    pub headers: Vec<(String, Vec<u8>)>,
    pub body: Vec<u8>,
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

    Ok(Some(response.body(saved_res.body)))
}

pub async fn save_response(
    redis_conn: &mut MultiplexedConnection,
    key: &IdempotencyKey,
    http_res: HttpResponse,
) -> Result<HttpResponse, actix_web::Error> {
    let (res_head, body) = http_res.into_parts();
    let body = to_bytes(body).await.map_err(e500)?;

    let headers: Vec<(String, Vec<u8>)> = res_head
        .headers()
        .iter()
        .map(|(n, v)| (n.as_str().to_owned(), v.as_bytes().to_owned()))
        .collect();
    // FIX THIS !

    let res = SavedResponse {
        status_code: res_head.status().as_u16(),
        headers,
        body: Vec::new(),
    };
    if let Err(err) = redis_conn
        .set_ex(&key.0, serde_json::to_string(&res).map_err(e500)?, 900)
        .await
        .map_err(e500)
    {
        _ = rollback_precache(redis_conn, key).await?;
        return Err(err);
    }

    Ok(res_head.set_body(body).map_into_boxed_body())
}

pub async fn rollback_precache(
    redis_conn: &mut MultiplexedConnection,
    key: &IdempotencyKey,
) -> Result<(), actix_web::Error> {
    _ = redis_conn.del(&key.0).await.map_err(e500)?;
    Ok(())
}
