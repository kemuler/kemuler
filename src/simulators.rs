//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub mod enigo;

#[cfg(any(feature = "string-event-logger", test))]
pub mod string_event_logger;
