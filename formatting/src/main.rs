use tracing::warn;

fn main() {
    // This is the default subscriber set up.
    let subscriber = tracing_subscriber::fmt().finish();

    // This creates a scope for where a subscriber is valid.
    // I would not use this in production but it is useful for this example.
    tracing::subscriber::with_default(subscriber, || {
        warn!("Hello World");
    });

    // Lets do a very simple style. No time and no colours.
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .compact()
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        warn!("Hello World");
    });

    // Lets use a JSON Lines output.
    // Much nicer to parse and store.
    let subscriber = tracing_subscriber::fmt().json().finish();

    tracing::subscriber::with_default(subscriber, || {
        warn!("Hello World");
    })

    // Customization is infinite.
    // You could even write your own subscriber from the ground up.
}
