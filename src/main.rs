use invitation_serv::{config::Config, server::Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: NEED TELEMETRY AND I'M SURE BG WORKER
    let config = Config::get().expect("Failed to read configuration");

    // tasks should be loop
    let server_task = tokio::spawn(Server::build(config).await?.run());

    tokio::select! {
        // SHOULD LOG WITH TRACING
        o = server_task => println!("SERVER UNEXPECTEDLY CLOSED WITH: {:?}", o)
    };

    Ok(())
}
