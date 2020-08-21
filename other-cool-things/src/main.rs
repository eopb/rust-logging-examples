use spandoc::spandoc;
use tracing::info;

// Thing #1 Spandoc

// Spandoc is a more advance macro.
// It parses your code and converts comments starting with "SPANDOC:" to spans.
#[spandoc]
fn foo() {
    {
        /// SPANDOC: this comment will be converted into a span for this lexical scope.
        info!("event 1");
    }

    info!("event 2");
}

// Thing #2 I used tracing to print logs to the JS console under wasm.
// https://gitlab.com/efunb/dark-forest-nea/-/blob/master/client/src/console.rs

// Thing #3 I created a crate to create log structures for server requests.
// It creates warning spans for 400 http codes and errors for 500.
// It also show one syntax for creating spans in an async context with overlaps.
// https://github.com/ethanboxx/tide-tracing/blob/master/src/lib.rs
