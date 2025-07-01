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

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};

    use super::*;

    #[test]
    fn should_create_valid_user_session_from_valid_clerk_jwt() {
        let mut other_jwt = Map::new();
        other_jwt.insert("email".into(), Value::String("email@email.com".into()));

        let mock_jwt = ClerkJwt {
            azp: Some("http://localhost:3000".into()),
            exp: 1639398300,
            iat: 1639398272,
            iss: "https://clean-mayfly-62.clerk.accounts.dev".into(),
            nbf: 1639398220,
            sub: "user_1deJLArSTiWiF1YdsEWysnhJLLY".into(),
            sid: None,
            act: None,
            org: None,
            other: other_jwt,
        };

        let user_session: UserSession = mock_jwt.try_into().expect("Should have email");

        assert_eq!(user_session.email, "email@email.com");
    }

    #[test]
    fn should_return_anyhow_error_if_session_is_missing_additional_data() {
        let mock_jwt = ClerkJwt {
            azp: Some("http://localhost:3000".into()),
            exp: 1639398300,
            iat: 1639398272,
            iss: "https://clean-mayfly-62.clerk.accounts.dev".into(),
            nbf: 1639398220,
            sub: "user_1deJLArSTiWiF1YdsEWysnhJLLY".into(),
            sid: None,
            act: None,
            org: None,
            other: Map::new(),
        };

        let user_session: anyhow::Result<UserSession> = mock_jwt.try_into();

        assert!(user_session.is_err());
    }
}
