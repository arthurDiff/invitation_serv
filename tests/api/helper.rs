use invitation_serv::{
    config,
    telemetry::{EnvLevel, init_new_subscriber},
};
use sqlx::PgPool;

use once_cell::sync::Lazy;

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
}

impl TestApp {
    pub fn spawn() -> TestApp {
        Lazy::force(&TRACING);
        let config = {
            let mut c = config::Config::get().expect("Failed to load config");
            c.database.name = "invitation_test_db".into();
            c.server.port = 0;
            c
        };
        todo!()
    }
}
