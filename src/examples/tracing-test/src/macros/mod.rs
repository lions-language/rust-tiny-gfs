pub fn use_debug_derive() {
    crate::init_stdout();

    #[derive(Debug)]
    struct MyStruct {
        field: &'static str,
    }

    let my_struct = MyStruct {
        field: "Hello world!",
    };

    // `my_struct` will be recorded using its `fmt::Debug` implementation.
    tracing::event!(tracing::Level::INFO, greeting = ?my_struct);
    // tracing::info!("hello");
}
