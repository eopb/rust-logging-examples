use std::{thread, time::Duration};
use tracing::{error, info, Level};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        // JSON will probably work best for files.
        .json()
        // We can write our logs anywhere that implements rust standard Writer trait.
        // Here we are creating a folder and adding log files in the form `example.log.yyyy-MM-dd-HH-mm`.
        // A new log file is created every minute to prevent huge log files.
        //
        // Other things we could write too include: Stdout, TCP streams or even the javascript
        // console with wasm.
        .with_writer(|| {
            tracing_appender::rolling::minutely(
                /* directory */ "logs",
                /* file_name_prefix */ "example.log",
            )
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    // Lets log in a loop every second.
    loop {
        info!("Now everything is going fine 🎉");
        error!("oh no everything went wrong 💣");

        thread::sleep(Duration::from_secs(1))
    }
}
