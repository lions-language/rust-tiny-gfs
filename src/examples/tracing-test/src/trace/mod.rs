pub fn print_trace() {
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

    {
        let span = tracing::span!(tracing::Level::INFO, "my span");

        tracing::event!(parent: &span, tracing::Level::INFO, "hello");
        tracing::event!(parent: &span, tracing::Level::INFO, "world");

        tracing::info!("my span");
    }
}

pub fn print_trace_v2() {
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

    {
        let span = tracing::span!(tracing::Level::INFO, "my span");

        let _enter = span.enter();

        tracing::event!(tracing::Level::INFO, "hello");
        tracing::event!(tracing::Level::INFO, "world");

        tracing::info!("my span");
    }
}
