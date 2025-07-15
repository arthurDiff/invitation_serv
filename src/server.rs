use actix_web::{App, HttpServer, dev::Server as ActixServer, middleware::from_fn, web};
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};
use redis::Client as RedisClient;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::{
    config::{Config, DatabaseConfig},
    routes::{create_event, get_event, get_events, health_check},
};

pub struct Server {
    port: u16,
    server: ActixServer,
}

impl Server {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        // dep inj
        let db_pool = get_connection_pool(&config.database);
        let redis = RedisClient::open(config.redis_url.expose_secret())?;
        let clerk = Clerk::new(ClerkConfiguration::new(
            None,
            None,
            Some(config.clerk_key.expose_secret().into()),
            None,
        ));
        // end dep inj

        let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
        let port = listener.local_addr()?.port();

        Ok(Self {
            port,
            server: run(listener, db_pool, redis, clerk).await?,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    redis: RedisClient,
    clerk: Clerk,
) -> Result<ActixServer, anyhow::Error> {
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .service(
                // Protected
                web::scope("/api")
                    .wrap(ClerkMiddleware::new(
                        MemoryCacheJwksProvider::new(clerk.clone()),
                        None,
                        true,
                    ))
                    .service(
                        web::scope("/events")
                            .route("", web::get().to(get_events))
                            .route("", web::post().to(create_event))
                            .route("/{event_id}", web::get().to(get_event)),
                    ),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis.clone()))
    })
    .listen(listener)?
    .run())
}

fn get_connection_pool(config: &DatabaseConfig) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}

/*
Need:
- rate limiting
- idempotency
- source block
- storage provider S3
- mailing service
*/

/*
Routes:
/events
/events/:eventId
/events/:eventId/invites
/group

*/
