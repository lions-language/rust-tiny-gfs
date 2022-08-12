fn create_appender_log<T>(name: &str, dir: &str, f: impl FnOnce() -> T) {
    use tracing_subscriber::fmt::time::LocalTime;

    let file_appender = crate::custom_rolling::hourly(dir, name);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_timer(LocalTime::rfc_3339())
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, f);
}

pub fn test() {
    create_appender_log("log1", ".logs", || {
        tracing::info!("hello 1");
    });
}
