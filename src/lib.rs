//! # Kemuler
//!
//! Welcome to an experimental input simulation crate, `kemuler`!
//!
//! This crate offers a different kind of high level input simulator framework,
//! utilizing Rust’s type system to its full potential (or just over-engineered)
//! instead of the usual key_down(Key).
//! Why?
//! Combinators!
//!
//! Current features:
//! - Multiple backends support (called `Simulator` in here).
//!   Built-ins:
//!   - Enigo (The crate that helped me make this crate and it is cross-platform.)
//!   - WinDirect (support for Window's DirectX/DirectInput something)
//! - Combinator
//!
//! Some drawbacks:
//! - Bunch of boilerplate is currently needed. (working on it)
//! - Combinator currently can only combine for the same `Simulator`
//!   Seperated branch for any `Simulator` combinator is on GitHub.
//! - Only a few amount of combinators is present.
//!   If you got some more useful combinator, please submit an issue on `GitHub`!
//!
//! # Examples
//! These examples requires the feature "enigo" to be enabled.
//!
//! ## Basic
//!
#![cfg_attr(feature = "enigo", doc = "```")]
#![cfg_attr(not(feature = "enigo"), doc = "```ignore")]
//! use kemuler::prelude::*;
//! use kemuler::simulators::Enigo;
//!
//! let e = Enigo::new();
//! // method 1
//! Keyboard::A.down().run_with(&mut e);
//!
//! // method 2
//! // The conventional style is still supported but not with combinators
//! // Consider other crates if you like this method :(. (such as `enigo`)
//! e.run(Keyboard::A.down());
//! ```
//!
//! ## Combinators
//!
#![cfg_attr(feature = "enigo", doc = "```")]
#![cfg_attr(not(feature = "enigo"), doc = "```ignore")]
//! use kemuler::prelude::*;
//! use kemuler::simulator::Enigo;
//!
//!
//! ```
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod combinator;
pub mod simulatable;
pub mod simulator;

pub mod inputs;
pub mod simulators;

pub mod prelude {
    //! Re-exports

    use super::*;

    pub use combinator::Combine;
    pub use inputs::common::*;
    pub use simulatable::Simulatable;
}
