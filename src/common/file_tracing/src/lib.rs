mod custom_rolling;

pub fn create_appender_log<T>(name: &str, dir: &str, f: impl FnOnce() -> T) -> T {
    use tracing_subscriber::fmt::time::LocalTime;

    let file_appender = custom_rolling::hourly(dir, name);

    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking_appender)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_timer(LocalTime::rfc_3339())
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, f)
}
