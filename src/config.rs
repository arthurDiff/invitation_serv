use config as cfg;
use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

pub enum Environment {
    Local,
    Prod,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a supported environemtn. Use either 'local' or 'prod'.",
                other
            )),
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis_url: SecretString,
    pub clerk_key: SecretString,
}

impl Config {
    pub fn get() -> Result<Config, config::ConfigError> {
        let config_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("configuration");

        Self::get_from(config_dir)
    }
    pub fn get_from(config_dir: std::path::PathBuf) -> Result<Config, cfg::ConfigError> {
        let env: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENV");

        cfg::Config::builder()
            .add_source(cfg::File::from(config_dir.join("base.yaml")))
            .add_source(cfg::File::from(
                config_dir.join(format!("{}.yaml", env.as_str())),
            ))
            .add_source(
                cfg::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .map(|c| c.try_deserialize::<Config>())?
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: SecretString,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: SecretString,
    pub require_ssl: bool,
}

impl DatabaseConfig {
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
        self.without_db().database(&self.name)
    }
}
