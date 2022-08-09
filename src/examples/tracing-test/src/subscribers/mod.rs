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
