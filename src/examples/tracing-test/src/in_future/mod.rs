pub fn print_in_tokio_runtime() {
    std::thread::spawn(move || {
        crate::library::create_appender_log(
            "async_tasks",
            "logs/chunk_handler",
            move || -> Result<(), String> {
                use tokio::runtime::Runtime;
                let rt = Runtime::new().unwrap();

                rt.block_on(async {});

                Ok(())
            },
        )
    });
}
