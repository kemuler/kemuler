//! # Kemuler
//!
//! This is an experimental input simulation/emulation crate with different approach.
//! This approach allows you to use combinator and switches backends (different impl for the same input).
//! Input (Called `Simulate` in library) will not do anything unless execute.
//!
//! # Examples
//! ## Basic
//!
//! ```
//! use kemuler::prelude::*;
//!
//! // Requires the feature "enigo"
//! use kemuler::simulate::enigo::Enigo;
//!
//! Key::A.set_to(false).simulate_with(&Enigo).execute();
//!
//! // Uses default simulator (later)
//! // Key::A.set_to(false).execute();
//! ```
//!
//! ## Combinator
//!
//! ```
//! use kemuler::prelude::*;
//! use std::time::Duration;
//!
//! // Requires the feature "enigo"
//! use kemuler::simulate::enigo::Enigo;
//!
//! let a = Key::A.set_to(false).simulate_with(&Enigo);
//! let b = Key::B.set_to(true).simulate_with(&Enigo);
//! let a_and_b_and_a = a
//!     .and_then(b)
//!     .and_sleep(Duration::from_micros(200))
//!     .execute();
//! ```

pub mod execute;
pub mod input;
pub mod simulate;

pub mod prelude {
    use super::*;

    pub use execute::{Combinator, Executable};
    pub use input::{common::Key, Input};
}
