fn create_appender_log<T>(name: &str, dir: &str, f: impl FnOnce() -> T) {
    let file_appender = tracing_appender::rolling::hourly(dir, name);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, f);
}

fn test() {
    create_appender_log("log1", ".logs", || {
        tracing::info!("hello 1");
    });
}
