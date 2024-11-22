pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into())
                .add_directive("hyper=off".parse().unwrap())
                .add_directive("reqwest=off".parse().unwrap()),
        )
        .init();
}

pub mod agent;
pub mod character;
pub mod discord;
pub mod knowledge;
pub mod loaders;
pub mod stores;