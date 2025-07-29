use std::thread;

use invite_server::{
    config::{self, Config, DatabaseConfig},
    server::Server,
    telemetry::{EnvLevel, init_new_subscriber},
};
use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};

use once_cell::sync::{Lazy, OnceCell};
use uuid::Uuid;

//TODO: NEED TO HAVE TEST SPECIFIC REDIS GROUP TOO

static TRACING: Lazy<()> = Lazy::new(|| {
    let (name, env_filter) = ("test_invite", EnvLevel::Debug);
    if std::env::var("TEST_LOG").is_ok() {
        init_new_subscriber(name, env_filter, std::io::stdout);
    } else {
        init_new_subscriber(name, env_filter, std::io::sink);
    }
});

pub static SESSION_TOKEN: OnceCell<String> = OnceCell::new();

#[allow(dead_code)]
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

        SESSION_TOKEN.get_or_init(|| get_user_session(config.clerk_key.expose_secret().into()));

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
            address: format!("http://127.0.0.1:{port}"),
            db_pool,
            api_client,
            config,
        }
    }
}

//? SINCE I DROP DOCKER INSTANCE SO OFTEN I WON'T CLEAN UP
// impl Drop for TestApp {
//     fn drop(&mut self) {
//         let db_config = self.config.database.clone();
//         let db_conn = self.db_pool.clone();
//         thread::spawn(move || {
//             tokio::runtime::Runtime::new().unwrap().block_on(async {
//                 // close current connection to db;
//                 db_conn.close().await;
//                 // drop the created table;
//                 cleanup_db(db_config).await;
//             })
//         })
//         .join()
//         .unwrap_or_else(|e| panic!("Failed cleaning up test db with error: {:?}", e));
//     }
// }

// async fn cleanup_db(config: DatabaseConfig) {
//     let mut conn = PgConnection::connect_with(&config.without_db())
//         .await
//         .expect("Failed to form db connection for cleanup");
//     conn.execute(format!(r#"DROP DATABASE "{}";"#, config.name).as_str())
//         .await
//         .expect("Failed dropping test db");
// }

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

#[allow(dead_code)]
const TEST_USER: &str = "user_2ysgqtTDuD2gPviOwynN5mlL36f";

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct TestSession {
    id: String,
    #[serde(flatten)]
    res: serde_json::Map<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct TestSessionToken {
    jwt: String,
    #[serde(flatten)]
    res: serde_json::Map<String, serde_json::Value>,
}

fn get_user_session(clerk_key: String) -> String {
    let client = reqwest::Client::new();

    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .expect("Failed to init tokio runtime to get session token")
            .block_on(async {
                let test_this = client
                    .post("https://api.clerk.com/v1/sessions")
                    .header("Authorization", format!("Bearer {clerk_key}"))
                    .json(&serde_json::json!({"user_id": TEST_USER}))
                    .send()
                    .await
                    .expect("Failed to get user session")
                    .json::<serde_json::Value>();
                panic!("{:?} WTF THIS IS THE VALUE", test_this);
                let test_session = client
                    .post("https://api.clerk.com/v1/sessions")
                    .header("Authorization", format!("Bearer {clerk_key}"))
                    .json(&serde_json::json!({"user_id": TEST_USER}))
                    .send()
                    .await
                    .expect("Failed to get user session")
                    .json::<TestSession>()
                    .await
                    .unwrap_or_else(|err| panic!("Failed to deserialize user session with err {:?}", err));

                client
                    .post(format!("https://api.clerk.com/v1/sessions/{}/tokens", test_session.id))
                    .header("Authorization", format!("Bearer {clerk_key}"))
                    .json(&serde_json::json!({"expires_in_seconds": 600}))
                    .send()
                    .await
                    .expect("Failed to get session token")
                    .json::<TestSessionToken>()
                    .await
                    .unwrap_or_else(|err| panic!("Failed to deserialize user session with err {:?}", err))
                    .jwt
            })
    })
    .join()
    .unwrap_or_else(|e| panic!("Failed cleaning up test db with error: {:?}", e))
}
