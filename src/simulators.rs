//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub use self::enigo::Enigo;

#[cfg(feature = "enigo")]
mod enigo;

#[cfg(feature = "event-log")]
pub use event_log::{EventLog, EventLogger};

#[cfg(feature = "event-log")]
mod event_log;
