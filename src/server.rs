use actix_web::{App, HttpServer, dev::Server as ActixServer, web};
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
};
use redis::aio::MultiplexedConnection;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::{
    config::Config,
    routes::{create_event, get_event, get_events, health_check},
};

pub struct Server {
    port: u16,
    server: ActixServer,
}

impl Server {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        // dep inj start
        let db_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
        let redis_conn = redis::Client::open(config.redis_url)?
            .get_multiplexed_async_connection()
            .await?;
        let clerk = Clerk::new(ClerkConfiguration::new(
            None,
            None,
            Some(config.clerk_key.expose_secret().into()),
            None,
        ));
        // dep inj end

        let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
        let port = listener.local_addr()?.port();

        Ok(Self {
            port,
            server: run(listener, db_pool, redis_conn, clerk).await?,
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
    redis_conn: MultiplexedConnection,
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
            .app_data(web::Data::new(redis_conn.clone()))
    })
    .listen(listener)?
    .run())
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
