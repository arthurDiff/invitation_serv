type UserId = String;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UserSession(UserId);

impl actix_web::FromRequest for UserSession {
    type Error = actix_web::Error;

    type Future = std::future::Ready<Result<UserSession, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let Some(session_cookie) = req.cookie("__session") else {
            todo!()
            // return Err()
        };
        todo!()
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
