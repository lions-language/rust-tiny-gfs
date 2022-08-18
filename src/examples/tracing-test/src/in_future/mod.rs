/// OK
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

/// OK
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

/// OK
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

/// OK
fn in_tokio_multi_sleep(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let sleep1 = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
                tokio::pin!(sleep1);

                let sleep2 = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
                tokio::pin!(sleep2);

                loop {
                    tokio::select! {
                        _ = &mut sleep1 => {
                            info!("log 2");
                            sleep1.as_mut().reset(tokio::time::Instant::now() + tokio::time::Duration::from_millis(1000));
                        }
                        _ = &mut sleep2 => {
                            info!("log 3");
                            sleep2.as_mut().reset(tokio::time::Instant::now() + tokio::time::Duration::from_millis(1000));
                        }
                    }
                }
            });

            Ok(())
        })
    })
    .join();
}

pub fn in_tokio_multi_sleep_stdout() {
    in_tokio_multi_sleep(crate::library::create_stdout());
}

pub fn in_tokio_multi_sleep_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    in_tokio_multi_sleep(w);
}

/// Failed
fn use_spawn(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                // NOTE: print `log 1` only
                for i in 0..10 {
                    tokio::spawn(async move {
                        info!("log {}", i);
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    });
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            });

            Ok(())
        })
    })
    .join();
}

pub fn use_spawn_stdout() {
    use_spawn(crate::library::create_stdout());
}

pub fn use_spawn_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    use_spawn(w);
}

/// Unknown
fn use_local_runtime(
    w: impl for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + 'static + Send + Sync,
) {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_log(w, move || -> Result<(), String> {
            info!("log 1");

            use tokio::runtime::Runtime;
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                // NOTE: print `log 1` only
                for i in 0..10 {
                    tokio::spawn(async move {
                        info!("log {}", i);
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    });
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            });

            Ok(())
        })
    })
    .join();
}

pub fn use_local_runtime_stdout() {
    use_local_runtime(crate::library::create_stdout());
}

pub fn use_local_runtime_file() {
    let (w, _g) = crate::library::create_appender("log1", ".logs");
    use_local_runtime(w);
}
