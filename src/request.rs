use actix_web::{error::PayloadError, mime, web};
use serde::de::DeserializeOwned;
use tokio_stream::StreamExt;

const MAX_PAYLOAD_SIZE: usize = 1024 * 5; // max payload size is 256k but set to 5MB

pub fn get_json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        // 5MB max
        .limit(1024 * 5)
        .content_type(|mime| mime == mime::APPLICATION_JSON)
        .error_handler(|err, _| actix_web::error::ErrorBadRequest(err))
}

pub trait RequestDeserializer {
    fn deserialize_to<T>(self) -> impl std::future::Future<Output = Result<T, anyhow::Error>>
    where
        T: DeserializeOwned;
}

impl RequestDeserializer for web::Payload {
    /// convert payload as json deserialized  type
    async fn deserialize_to<T>(mut self) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let mut buf = web::BytesMut::new();
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            if buf.len() + chunk.len() > MAX_PAYLOAD_SIZE {
                return Err(PayloadError::Overflow.into());
            }
            buf.extend_from_slice(&chunk);
        }

        Ok(serde_json::from_slice::<T>(&buf)?)
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{FromRequest, test::TestRequest, web::Payload};

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestJson {
        hi: String,
        bye: String,
    }

    #[tokio::test]
    async fn should_deserialize_correctly_when_payload_is_valid() {
        let test_payload = TestRequest::default()
            .set_json(TestJson {
                hi: "world".into(),
                bye: "nay".into(),
            })
            .to_request()
            .take_payload();

        todo!()
    }
}
