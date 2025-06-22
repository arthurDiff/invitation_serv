use invitation_serv::{
    config::{self, Config, DatabaseConfig},
    server::Server,
    telemetry::{EnvLevel, init_new_subscriber},
};
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};

use once_cell::sync::Lazy;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let (name, env_filter) = ("test_invitation", EnvLevel::Debug);
    if std::env::var("TEST_LOG").is_ok() {
        init_new_subscriber(name, env_filter, std::io::stdout);
    } else {
        init_new_subscriber(name, env_filter, std::io::sink);
    }
});

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: Client,
    pub config: Config,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        Lazy::force(&TRACING);
        let config = {
            let mut c = config::Config::get().expect("Failed to load config");
            c.database.name = Uuid::new_v4().to_string();
            c.server.port = 0;
            c
        };
        let db_pool = configure_db(&config.database).await;
        let test_serv = Server::build(config.clone())
            .await
            .expect("Failed creating test server instance");

        let port = test_serv.port();
        tokio::spawn(test_serv.run());

        let api_client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .expect("Failed building test api client");

        TestApp {
            port,
            address: format!("http://127.0.0.1:{}", port),
            db_pool,
            api_client,
            config,
        }
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let db_name = &self.config.database.name;
        let cleanup = tokio::runtime::Runtime::new().unwrap().block_on(
            self.db_pool
                .execute(format!(r#"DROP DATABASE "{db_name}""#).as_str()),
        );
        cleanup.unwrap_or_else(|e| {
            panic!(
                "Failed cleaning up test db with name: {} with error: {}",
                db_name, e
            )
        });
    }
}

async fn configure_db(config: &DatabaseConfig) -> PgPool {
    // Create a new db for test
    let mut conn = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to form test db connection");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed creating test database");

    // Create db pool conn
    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to test postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate to test db");

    db_pool
}
