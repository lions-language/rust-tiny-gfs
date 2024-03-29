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
    tracing::event!(tracing::Level::INFO, greeting = ?my_struct, field = my_struct.field);
    // tracing::info!("hello");
}

pub fn use_display_derive() {
    crate::init_stdout();

    use std::fmt;

    struct MyStruct {
        field: &'static str,
    }

    impl fmt::Display for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.field)
        }
    }

    let my_struct = MyStruct {
        field: "Hello world!",
    };

    // display for struct field
    tracing::event!(tracing::Level::INFO, greeting = %my_struct.field);

    // display for struct
    tracing::event!(tracing::Level::INFO, greeting = %my_struct);
}

pub fn build_span_use_empty() {
    crate::init_stdout();

    let span = tracing::info_span!("span 1", xxx = tracing::field::Empty);

    span.record("xxx", &"hello 1");

    tracing::event!(parent: &span, tracing::Level::INFO, "hello 2");
}

pub fn use_kv_in_event() {
    crate::init_stdout();

    let name = "Mike";

    tracing::event!(
        tracing::Level::INFO,
        xxx1 = "xxx1",
        xxx2 = "xxx2",
        "hello {}",
        name
    );

    // 2022-08-09T15:10:23.969792Z  INFO tracing_test::macros: hello Mike xxx1="xxx1" xxx2="xxx2"
}
