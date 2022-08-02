use tracing::instrument::WithSubscriber;

pub fn spans_test() {
    use tracing::{span, Level};
    let span = span!(Level::TRACE, "my_span");
    // `enter` returns a RAII guard which, when dropped, exits the span. this
    // indicates that we are in the span for the current lexical scope.
    let _enter = span.enter();
}

pub fn spans_record_test() {
    use tracing::{field, trace_span};
    let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

    // ...

    // Now, record a value for parting as well.
    // (note that the field name is passed as a string slice)
    span.record("parting", "goodbye world!");
}
