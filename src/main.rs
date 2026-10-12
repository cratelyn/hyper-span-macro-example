//! A reproduction of hyper's tracing macros.
//!
//! Use this to explore ways to satiate linting rules, particularly `let-unit-value`.

#![deny(warnings)]
#![deny(clippy::all)]

/// Tracing macros, vendored from [`hyper::trace`].
///
/// [`hyper::trace`]: https://github.com/hyperium/hyper/blob/master/src/trace.rs
#[macro_use]
mod trace {
    macro_rules! info {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            {
                tracing::info!($($arg)+);
            }
        }
    }

    macro_rules! trace {
        ($($arg:tt)*) => {
            #[cfg(feature = "tracing")]
            {
                tracing::trace!($($arg)+);
            }
        }
    }

    macro_rules! trace_span {
        ($($arg:tt)*) => {
            {
                #[cfg(feature = "tracing")]
                {
                    let _span = tracing::trace_span!($($arg)+);
                    _span.entered()
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Log an event before and after entering a span.
    info!("before entering span");
    let _entered = trace_span!("instrumented");
    info!("after entering span");

    // Call a function, which will log an event.
    instrumented();
}

fn instrumented() {
    trace!("within an instrumented function");
}
