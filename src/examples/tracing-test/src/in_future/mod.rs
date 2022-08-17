fn tokio_runtime(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                info!("log 2");
            });

            Ok(())
        })
    })
    .join();
}

pub fn tokio_runtime_to_stdout() {
    tokio_runtime(crate::library::create_stdout());
}

pub fn tokio_runtime_to_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    tokio_runtime(w);
}

fn in_await(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let f = async {
        info!("log 3");
    };

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                info!("log 2");

                f.await;
            });

            Ok(())
        })
    })
    .join();
}

pub fn in_await_to_stdout() {
    in_await(crate::library::create_stdout());
}

pub fn in_await_to_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    in_await(w);
}

fn in_tokio_sleep(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
                tokio::pin!(sleep);

                loop {
                    tokio::select! {
                        _ = &mut sleep => {
                            info!("log 2");
                            sleep.as_mut().reset(tokio::time::Instant::now() + tokio::time::Duration::from_millis(1000));
                        }
                    }
                }
            });

            Ok(())
        })
    })
    .join();
}

pub fn in_tokio_sleep_stdout() {
    in_tokio_sleep(crate::library::create_stdout());
}

pub fn in_tokio_sleep_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    in_tokio_sleep(w);
}
