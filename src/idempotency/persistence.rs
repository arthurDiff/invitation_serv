use actix_web::{HttpResponse, http::StatusCode};
use redis::TypedCommands;
use serde_json::Value as JsonValue;

use crate::{idempotency::IdempotencyKey, response::ResponseBody};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SavedResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: JsonValue,
}

const PLACEHOLDER_IDEM_VALUE: &str = "nil";
pub fn try_get_response(redis: &redis::Client, key: &IdempotencyKey) -> Result<Option<HttpResponse>, anyhow::Error> {
    let mut conn = redis.get_connection()?;
    let Some(saved_res) = conn.get(&key.0)? else {
        // 60 * 15 | 15m
        let _: () = conn.set_ex(&key.0, PLACEHOLDER_IDEM_VALUE, 900)?;
        return Ok(None);
    };

    if saved_res == PLACEHOLDER_IDEM_VALUE {
        // return 409 if operation is already in progress
        return Ok(Some(HttpResponse::build(StatusCode::CONFLICT).json(
            ResponseBody::<()> {
                success: false,
                message: "Operation is already in progress.".into(),
                data: None,
            },
        )));
    }

    let saved_res = serde_json::from_str::<SavedResponse>(&saved_res)?;

    let mut response = HttpResponse::build(StatusCode::from_u16(saved_res.status_code)?);

    saved_res.headers.into_iter().for_each(|(k, v)| {
        response.insert_header((k, v));
    });

    Ok(Some(response.json(saved_res.body)))
}
