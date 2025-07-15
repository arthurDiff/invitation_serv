mod persistence;

use actix_web::http::header::HeaderValue;

pub use persistence::*;
#[derive(Debug)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn attach(self, op_name: &str, user_id: &str) -> Self {
        IdempotencyKey(format!("{}__{op_name}__{user_id}", &self.0))
    }
}

impl TryFrom<&HeaderValue> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(hv: &HeaderValue) -> Result<Self, Self::Error> {
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
