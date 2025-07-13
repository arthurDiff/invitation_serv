use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::idempotency::IdempotencyKey;

const IDEMPOTENCY_HEADER_KEY: &str = "idempotency-key";

//TODO: attach user_id to the key

pub async fn attach_idempotency_key(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let Some(header_idem_key) = req.headers().get(IDEMPOTENCY_HEADER_KEY) else {
        return next.call(req).await;
    };

    match IdempotencyKey::try_from(header_idem_key) {
        Ok(key) => {
            req.extensions_mut().insert(key);
            next.call(req).await
        }
        Err(_) => Err(actix_web::error::ErrorBadRequest(anyhow::anyhow!(
            "Invalid idempotency key type provided in header."
        ))),
    }
}
