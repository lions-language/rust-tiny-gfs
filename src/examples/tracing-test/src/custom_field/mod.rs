pub fn custom_field() {
    use tracing_subscriber::{fmt::format, prelude::*};

    let format =
        format::debug_fn(|writer, field, value| write!(writer, "{} => {:?}", field, value))
            .delimited(", ");

    tracing_subscriber::fmt().fmt_fields(format).init();

    tracing::info!("hello");
}

pub fn use_local_time() {
    use tracing_subscriber::{fmt::format, prelude::*};

    let format =
        format::debug_fn(|writer, field, value| write!(writer, "{} => {:?}", field, value))
            .delimited(", ");

    tracing_subscriber::fmt().fmt_fields(format).init();

    tracing::info!("hello");
}
