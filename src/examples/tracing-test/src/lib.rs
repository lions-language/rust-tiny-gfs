pub mod basic;
pub mod custom_field;
pub mod macros;
pub mod simple_file;
pub mod subscribers;
pub mod trace;

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
