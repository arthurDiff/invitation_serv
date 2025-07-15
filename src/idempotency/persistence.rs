use actix_web::{HttpResponse, http::StatusCode};
use redis::Commands;
use serde_json::Value as JsonValue;

use crate::idempotency::IdempotencyKey;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SavedResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: JsonValue,
}

pub fn try_get_response(redis: &redis::Client, key: &IdempotencyKey) -> Result<Option<HttpResponse>, anyhow::Error> {
    let mut conn = redis.get_connection()?;
    let cached_res: redis::Value = conn.get(&key.0)?;

    let saved_res = match cached_res {
        // operation already registered return result
        redis::Value::SimpleString(val) => serde_json::from_str::<SavedResponse>(&val)?,
        // no operation present reserve by setting kv
        redis::Value::Nil => {
            let _: redis::Value = conn.set(&key.0, "NULL")?;
            return Ok(None);
        }
        // operation is already getting performed
        // TODO: return 409 response
        redis::Value::Okay => return Ok(None),
        _ => anyhow::bail!("Invalid cached response"),
    };

    let mut response = HttpResponse::build(StatusCode::from_u16(saved_res.status_code)?);

    saved_res.headers.into_iter().for_each(|(k, v)| {
        response.insert_header((k, v));
    });

    Ok(Some(response.json(saved_res.body)))
}
