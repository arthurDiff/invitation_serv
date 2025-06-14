use actix_web::dev::Server as ActixServer;
use std::net::TcpListener;

pub struct Server {
    port: u16,
}

impl Server {
    pub async fn build() -> Result<Self, anyhow::Error> {
        todo!()
    }
}

async fn run(listener: TcpListener) -> Result<ActixServer, anyhow::Error> {
    todo!();
}
