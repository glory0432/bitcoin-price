use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
pub fn subscribe_tracing() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true),
        )
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
                .add_directive("sqlx=off".parse().unwrap())
                .add_directive("hyper=warn".parse().unwrap())
                .add_directive("axum=info".parse().unwrap()),
        )
        .init();
}
