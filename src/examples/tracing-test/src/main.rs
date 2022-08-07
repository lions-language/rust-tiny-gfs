use tracing_test::basic;
use tracing_test::custom_field;
use tracing_test::simple_file;
use tracing_test::trace;

fn main() {
    // basic::spans_test();
    // basic::event_test();

    // custom_field::custom_field();
    // custom_field::use_local_time();

    // simple_file::global_config_to_file();

    // trace::print_trace();
    // trace::print_trace_use_enter();
    // trace::print_trace_in_scope();
    trace::use_instrument();
}
