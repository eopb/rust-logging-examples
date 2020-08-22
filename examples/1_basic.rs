use tracing::{debug, error, info, trace, warn, Level};

fn main() {
    // The tracing crate uses the facade pattern so we need to use the tracing_subscriber crate to
    // grab the logs, format them and write them somewhere.
    let subscriber = tracing_subscriber::fmt()
        // This max_level setting is set using something called the builder pattern.
        // Rust uses the builder pattern because it does not have optional arguments.
        .with_max_level(Level::TRACE)
        .finish();

    // Subscribers can be used for lots of things. This line set up a subscriber globally.
    //
    // The expect method is a standard way of handling errors in Rust.
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    // This is an event. Events may be caught by a subscriber.
    trace!("Something not very interesting happened.");

    // Rust does not have first class support of variadic functions.
    // Rust often uses macros to simulate the same experience with extra powers.
    // All macros have a `!` after their name.
    debug!(
        some_interesting_num = 5,
        "Someone trying to debug 🐛 this code may find this interesting",
    );

    info!("Just want to mention what my program is doing right now 👀");

    warn!("Program does not feel so good 🤢");

    error!("Oh no this code is falling apart 🤮");
}
