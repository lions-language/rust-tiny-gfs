use tracing::{event, instrument::WithSubscriber};

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

    let _enter = span.enter();

    // ...

    // Now, record a value for parting as well.
    // (note that the field name is passed as a string slice)
    // span.record("parting", false);
}

pub fn event_test() {
    use tracing::{event, span, Level};
    let span = span!(Level::TRACE, "my_span");
    let _enter = span.enter();
    event!(parent: &span, Level::INFO, "hello");
}
