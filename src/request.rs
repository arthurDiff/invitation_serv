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
    fn deserialize_as<T>(self) -> impl std::future::Future<Output = Result<T, anyhow::Error>>
    where
        T: DeserializeOwned;
}

impl RequestDeserializer for web::Payload {
    /// convert payload as json deserialized  type
    async fn deserialize_as<T>(mut self) -> Result<T, anyhow::Error>
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
    use actix_web::{
        FromRequest, dev,
        test::TestRequest,
        web::{Bytes, Payload},
    };

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestJson {
        hi: String,
        bye: String,
    }

    #[tokio::test]
    async fn should_deserialize_correctly_when_payload_is_valid() {
        let mut dev_payload = dev::Payload::from(Bytes::from(r#"{"hi":"world","bye":"nay","something":"dropped"}"#));

        let payload: Payload = web::Payload::from_request(&TestRequest::default().to_http_request(), &mut dev_payload)
            .await
            .expect("Expect payload to exists");

        let json = payload
            .deserialize_as::<TestJson>()
            .await
            .expect("Should have deserialized appropriately");

        assert_eq!(json.hi, "world");
        assert_eq!(json.bye, "nay");
    }

    #[tokio::test]
    async fn should_fail_deserialization_if_req_payload_is_wrong() {
        let mut dev_payload = dev::Payload::from(Bytes::from(r#"{"hworld":"world","bye":"nay"}"#));

        let payload: Payload = web::Payload::from_request(&TestRequest::default().to_http_request(), &mut dev_payload)
            .await
            .expect("Expect payload to exists");

        let json = payload.deserialize_as::<TestJson>().await;

        assert!(json.is_err());
    }
}
