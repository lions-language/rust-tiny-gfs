pub fn global_config_to_file() {
    use tracing_subscriber::{fmt::format, prelude::*};

    // Format fields using the provided closure.
    let format = format::debug_fn(|writer, field, value| {
        // We'll format the field name and value separated with a colon.
        write!(writer, "{}: {:?}", field, value)
    })
    // Separate each field with a comma.
    // This method is provided by an extension trait in the
    // `tracing-subscriber` prelude.
    .delimited(", ");

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().fmt_fields(format).init();

    tracing::info!("preparing to shave yaks");
}
