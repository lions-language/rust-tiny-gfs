pub fn global_config_to_file() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let file_appender = tracing_appender::rolling::hourly(".logs/", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        // merge stdio and file
        .with_writer(stdout.and(non_blocking))
        // disenable ANSI terminal colors for formatted output
        .with_ansi(false)
        .init();

    tracing::info!("hello");
}
