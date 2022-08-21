pub fn use_with_default() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .finish();

    let my_dispatch = tracing::Dispatch::new(subscriber);
    tracing::dispatcher::with_default(&my_dispatch, || {
        // tracing::subscriber::with_default(subscriber, || {
        tracing::info!("hello 1");
        // });
    });
}

pub fn tracing_future_with_default() {
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .finish();

    let my_dispatch = tracing::Dispatch::new(subscriber);
    use tracing_futures::WithSubscriber;

    let dispatch = my_dispatch.with_current_subscriber();

    tracing::dispatcher::with_default(dispatch.dispatch(), || {
        // tracing::subscriber::with_default(subscriber, || {
        tracing::info!("hello 1");
        // });
    })
}
