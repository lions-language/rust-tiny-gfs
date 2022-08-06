use crate::basic::spans_record_test;

pub fn print_trace() {
    use tracing::{event, info, Span};
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let inner_fn = |span: &Span| {
        event!(parent: span, tracing::Level::INFO, "inner fn hello");
    };

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
        let span1 = tracing::span!(tracing::Level::INFO, "span-1");

        tracing::event!(parent: &span1, tracing::Level::INFO, "hello 1");
        tracing::event!(parent: &span1, tracing::Level::INFO, "hello 2");

        inner_fn(&span1);

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

pub fn print_trace_in_scope() {
    use tracing::{info, info_span};
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
        let span = tracing::span!(tracing::Level::INFO, "root-span");

        span.in_scope(|| {
            let inner_span = info_span!("span-1");
            // the span will be entered for the duration of the call to
            // `hello_world`.
            inner_span.in_scope(|| {
                info!("hello 1");

                let inner_span = info_span!("span-2");
                inner_span.in_scope(|| {
                    info!("hello 2");

                    let inner_span = info_span!("span-1");
                    inner_span.in_scope(|| {
                        info!("hello 3");
                    });
                });
            });

            info!("world 1");
        });
    }
}
