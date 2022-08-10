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

pub fn multi_simple_file_with_default() {
    {
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
    {
        let file_appender = tracing_appender::rolling::hourly(".logs/", "prefix_2.log");

        let subscriber = tracing_subscriber::fmt()
            .with_writer(file_appender)
            .with_max_level(tracing::Level::INFO)
            .with_ansi(false)
            .compact()
            .finish();

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("hello 2");
        });
    }
}

pub fn thread_in_with_default() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("hello 1");
        std::thread::spawn(|| {
            // NOTE: not show
            tracing::info!("hello in thread");
        });
    });
}
