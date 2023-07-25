//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub use self::enigo::{Enigo, EnigoKeyExt};

#[cfg(feature = "enigo")]
mod enigo;
