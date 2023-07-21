//! # Kemuler
//!
//! Welcome to an experimental input crate, `kemuler`!
//!
//! This crate offers a different kind of framework for simulating input
//! utilizing Rust's type system to its full potential (over engineered),
//! instead of the usual `key_down(Key)`.
//!
//! Features:
//! - Highly customizable:
//!     You can make any input work with any backend.
//!     (A backend is called simulator in here)
//! - Combinators:
//!     But only a few is implemented currently.
//!     Make a github issue if you got some more useful combinators!
//!
//! Some drawbacks:
//! - Combinator needed boxed dynamic dispatch which mean allocation (for it's full potential).
//! - Bunch of boilerplate is currently needed. (working on it)
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

pub mod combinator;
pub mod input;
pub mod simulate;

pub mod inputs;
pub mod simulators;

pub mod prelude {
    use super::*;

    pub use combinator::Combinator;
    pub use input::Input;
    pub use inputs::*;
    pub use simulate::Simulatable;
}
