use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize, Clone)]
pub struct Configs {
    pub database: DatabaseConfigs,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfigs {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub require_ssl: bool,
}

impl DatabaseConfigs {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Disable
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.db_name)
    }
}
