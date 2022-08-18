fn main() {
    // use tracing_test::basic;
    // basic::spans_test();
    // basic::event_test();

    // use tracing_test::custom_field;
    // custom_field::custom_field();
    // custom_field::use_local_time();

    // use tracing_test::simple_file;
    // simple_file::global_config_to_file();

    // use tracing_test::trace;
    // trace::print_trace();
    // trace::print_trace_use_enter();
    // trace::print_trace_in_scope();
    // trace::use_instrument();

    // use tracing_test::macros;
    // macros::use_debug_derive();
    // macros::use_display_derive();
    // macros::build_span_use_empty();
    // macros::use_kv_in_event();

    // use tracing_test::subscribers;
    // subscribers::use_with_default();
    // subscribers::simple_file_with_default();
    // subscribers::multi_simple_file_with_default();
    // subscribers::thread_in_with_default();

    // use tracing_test::library;
    // library::test();

    use tracing_test::in_future;
    // in_future::tokio_runtime_to_stdout();
    // in_future::tokio_runtime_to_file();
    // in_future::in_await_to_stdout();
    // in_future::in_await_to_file();
    // in_future::in_tokio_sleep_stdout();
    // in_future::in_tokio_sleep_file();
    // in_future::in_tokio_multi_sleep_stdout();
    // in_future::use_spawn_stdout()
    in_future::use_local_runtime_stdout();
}
