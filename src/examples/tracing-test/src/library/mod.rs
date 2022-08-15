pub fn create_log<W, T>(w: W, f: impl FnOnce() -> T) -> T
where
    W: for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
{
    use tracing_subscriber::fmt::time::LocalTime;

    let subscriber = tracing_subscriber::fmt()
        .with_writer(w)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_timer(LocalTime::rfc_3339())
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, f)
}

pub fn create_stdout_log<T>(f: impl FnOnce() -> T) -> T {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    create_log(stdout, f)
}

pub fn create_appender_log<T>(name: &str, dir: &str, f: impl FnOnce() -> T) -> T {
    let file_appender = crate::custom_rolling::hourly(dir, name);

    let (non_blocking_appender, _guard) = crate::custom_rolling::non_blocking(file_appender);

    create_log(non_blocking_appender, f)
}

pub fn test() {
    create_appender_log("log1", ".logs", || {
        tracing::info!("hello 1");
    });
}
