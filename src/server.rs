use actix_web::{App, HttpServer, dev::Server as ActixServer, web};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;

use crate::{
    config::{Config, DatabaseConfig},
    routes::health_check,
};

pub struct Server {
    port: u16,
    server: ActixServer,
}

impl Server {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let db_pool = get_connection_pool(&config.database);
        let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
        let port = listener.local_addr()?.port();
        Ok(Self {
            port,
            server: run(listener, db_pool).await?,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, db_pool: PgPool) -> Result<ActixServer, anyhow::Error> {
    let db_conn = web::Data::new(db_pool);
    Ok(HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .app_data(db_conn.clone())
    })
    .listen(listener)?
    .run())
}

fn get_connection_pool(config: &DatabaseConfig) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
