pub fn tokio_runtime_to_stdout() {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::create_stdout_log(move || -> Result<(), String> {
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

pub fn tokio_runtime_to_file() {
    use tracing::info;

    let _g = std::thread::spawn(move || {
        crate::library::create_appender_log("log1", ".logs", move || -> Result<(), String> {
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

pub fn await_to_stdout() {
    use tracing::info;

    let f = async {
        info!("log 3");
    };

    let _g = std::thread::spawn(move || {
        crate::create_stdout_log(move || -> Result<(), String> {
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
