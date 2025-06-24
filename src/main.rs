use invitation_serv::{
    config::Config,
    server::Server,
    telemetry::{EnvLevel, init_new_subscriber},
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_new_subscriber("invitation", EnvLevel::Info, std::io::stdout);
    // TODO: NEED BG WORKER FOR SOMETHING
    let config = Config::get().expect("Failed to read configuration");

    // tasks should be loop
    let server_task = tokio::spawn(Server::build(config).await?.run());

    tokio::select! {
        o = server_task => task_exit_report("API", o)
    };

    Ok(())
}

fn task_exit_report(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => tracing::info!("{} existed", task_name),
        Ok(Err(e)) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} failed", task_name)
        }
        Err(e) => {
            tracing::error!(erro.cause_chain = ?e, error.message = %e, "{} task failed to complete", task_name)
        }
    }
}
