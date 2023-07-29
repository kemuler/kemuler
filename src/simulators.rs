//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub mod enigo;

#[cfg(feature = "string-event-logger")]
pub mod string_event_logger;
