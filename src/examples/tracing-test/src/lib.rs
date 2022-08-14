pub mod basic;
pub mod custom_field;
pub mod in_future;
pub mod macros;
pub mod simple_file;
pub mod subscribers;
pub mod trace;

pub mod custom_rolling;
pub mod library;

fn init_stdout() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        // merge stdio and file
        .with_writer(stdout)
        // disenable ANSI terminal colors for formatted output
        .with_ansi(false)
        .init();
}

pub fn create_stdout_log<T>(f: impl FnOnce() -> T) -> T {
    use tracing_subscriber::fmt::time::LocalTime;

    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_timer(LocalTime::rfc_3339())
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, f)
}
