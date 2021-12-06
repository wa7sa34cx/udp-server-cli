//! Logging module
 
/// Enables logging through [pretty-env-logger].
///
/// A logger will **only** print errors, warnings, and general information
///
/// # Example
/// ```no_compile
/// enable_logging!();
/// ```
///
/// # Note
/// Calling this macro is **optional**
/// You can setup your own logger if you want.
///
/// [pretty-env-logger]: https://crates.io/crates/pretty_env_logger
#[macro_export]
macro_rules! enable_logging {
    () => {
        pretty_env_logger::formatted_builder()
            .write_style(pretty_env_logger::env_logger::WriteStyle::Auto)
            .filter(
                Some(&env!("CARGO_PKG_NAME").replace("-", "_")),
                log::LevelFilter::Trace,
            )
            .init();
    };
}
