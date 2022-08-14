pub fn print_in_tokio_runtime() {
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
