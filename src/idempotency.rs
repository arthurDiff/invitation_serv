mod persistence;

use actix_web::http::header::HeaderMap;

pub use persistence::*;
#[derive(Debug)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn attach(self, op_name: &str, user_id: &str) -> Self {
        IdempotencyKey(format!("{}__{op_name}__{user_id}", &self.0))
    }
}

impl TryFrom<&HeaderMap> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(header: &HeaderMap) -> Result<Self, Self::Error> {
        let Some(hv) = header.get("idempotency-key") else {
            return Ok(Self(uuid::Uuid::new_v4().to_string()));
        };

        let Ok(header_key) = hv.to_str() else {
            anyhow::bail!("idempotency-key header value is invalid format")
        };

        if header_key.is_empty() {
            anyhow::bail!("The idempotency key provided is empty string");
        }

        Ok(Self(header_key.to_string()))
    }
}

impl std::ops::Deref for IdempotencyKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
