use actix_web::http::header::HeaderValue;

///
/// Expected key format <op_name>-uuid
#[derive(Debug)]
pub struct IdempotencyKey(String);

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

pub struct Idempotency;

impl Idempotency {}
