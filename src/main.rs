use invitation_serv::{
    config::Config,
    server::Server,
    telemetry::{EnvLevel, init_new_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_new_subscriber("invitation", EnvLevel::Info, std::io::stdout);
    // TODO: NEED BG WORKER FOR SOMETHING
    let config = Config::get().expect("Failed to read configuration");

    // tasks should be loop
    let server_task = tokio::spawn(Server::build(config).await?.run());

    tokio::select! {
        // SHOULD LOG WITH TRACING
        o = server_task => println!("SERVER UNEXPECTEDLY CLOSED WITH: {:?}", o)
    };

    Ok(())
}
