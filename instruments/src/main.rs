use tracing::{info, instrument, Level};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        // Lets create a log message when ever a span is closed
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    info!("Starting up");

    info!("The 6th fibonacci number is {}", nthfib(6))
}

// instrument is a special kind of rust macro that acts a little like a python decorator.
// It sets up spans automatically for a function scope.

// By default it captures all of the functions arguments and logs them as key value pairs.
// It also works with async functions without weird overlapping scopes.
#[instrument(level = "trace")]
fn nthfib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => nthfib(n - 1) + nthfib(n - 2),
    }
}
