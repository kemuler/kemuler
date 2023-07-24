//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub use self::enigo::Enigo;

#[cfg(feature = "enigo")]
mod enigo;
