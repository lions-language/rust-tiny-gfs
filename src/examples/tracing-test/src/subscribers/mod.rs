pub fn use_with_default() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("hello 1");
    });
}

pub fn simple_file_with_default() {
    let file_appender = tracing_appender::rolling::hourly(".logs/", "prefix.log");

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("hello 1");
    });
}
