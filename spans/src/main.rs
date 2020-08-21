use tracing::{span, trace, Level};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    // The reason the tracing crate is so great is that it has great support for structural logging.

    trace!("This message is not part of a span.");

    // First we are going to create a scope for our span so our span is dropped by rust at the end of the scope.
    {
        let span = span!(Level::TRACE, "An interesting context");
        // `enter` returns a guard which, when dropped, exits the span.
        // We add an _ to tell rust that we intend to never use this variable
        // because the only thing interesting to do with it is dropping.
        let _enter = span.enter();

        trace!("This trace will be shown as part of an interesting context")
    } // Enter is dropped by rust here because the scope is closed. This exits the span.

    // We could also drop ourselves.

    // Spans can also have key value pairs.
    let span = span!(Level::TRACE, "A new context", key = "value");
    let enter = span.enter();

    trace!("This trace will be shown as part of a new context");

    drop(enter) // Enter is dropped manually, the scope is closed.
}
