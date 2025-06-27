use clerk_rs::validators::authorizer::ClerkJwt;

/// Structuring clerk session claims
/// https://clerk.com/docs/backend-requests/resources/session-tokens
pub struct UserSession {
    pub expires: i32,
    // sub
    pub user_id: String,
    pub email: String,
}

impl TryFrom<ClerkJwt> for UserSession {
    type Error = anyhow::Error;

    fn try_from(value: ClerkJwt) -> Result<Self, Self::Error> {
        let Some(email) = value.other.get("email").take_if(|v| v.is_string()) else {
            anyhow::bail!("Invalid session claim");
        };

        Ok(Self {
            expires: value.exp,
            user_id: value.sub,
            email: email
                .as_str()
                .expect("Session claim email property should be string")
                .into(),
        })
    }
}
