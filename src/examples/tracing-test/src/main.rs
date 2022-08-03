use tracing_test::basic;
use tracing_test::custom_field;
use tracing_test::simple_file;

fn main() {
    // basic::spans_test();
    // basic::event_test();
    // simple_file::global_config_to_file();
    // custom_field::custom_field();
    custom_field::use_local_time();
}
