use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt};

pub enum EnvLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl EnvLevel {
    pub fn as_str(&self) -> &str {
        match self {
            EnvLevel::Off => "off",
            EnvLevel::Error => "error",
            EnvLevel::Warn => "warn",
            EnvLevel::Info => "info",
            EnvLevel::Debug => "debug",
            EnvLevel::Trace => "trace",
        }
    }
}

pub fn generate_subscriber<Sink>(
    name: &str,
    env_filter: EnvLevel,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    //? Might want to add Sentry layer on production build
    //? https://crates.io/crates/sentry-tracing
    Registry::default()
        // ENV_FILTER
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter.as_str())))
        // Bunyan
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name.into(), sink))
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    tracing_log::LogTracer::init().expect("Failed to initialize log tracer");
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

pub fn init_new_subscriber<Sink>(name: &str, env_filter: EnvLevel, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    init_subscriber(generate_subscriber(name, env_filter, sink));
}
