//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub mod enigo;

#[cfg(test)]
pub(crate) mod string_event_logger;
