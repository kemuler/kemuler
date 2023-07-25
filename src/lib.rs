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
//! - Combinator currently can only combine for the same `Simulator`.
//!   Seperated branch for any `Simulator` combinator is on GitHub.
//! - Only a few amount of combinators is present.
//!   If you got some more useful combinator, please submit an issue on `GitHub`!
//!
//! # Examples
//! All these examples requires the feature "enigo".
//!
//! ## Basic
//!
#![cfg_attr(feature = "enigo", doc = "```")]
#![cfg_attr(not(feature = "enigo"), doc = "```ignore")]
//! use kemuler::prelude::*;
//! use kemuler::simulators::{Enigo, EnigoKeyExt};
//! use enigo::{Key, MouseButton};
//!
//! let mut enigo = Enigo::new();
//! // method 1
//! Key::Shift.down().run_with(&mut enigo);
//! Key::Shift.up().run_with(&mut enigo);
//!
//! // method 2
//! // The conventional style is still supported but not with combinators
//! use kemuler::simulator::Simulate;
//! enigo.simulate(MouseButton::Left.down());
//! enigo.simulate(MouseButton::Left.up());
//!
//! // Doesn't compiles!
//! // (`.click` uses combinator internally).
//! // enigo.simulate(MouseButton::Left.click());
//! ```
//!
//! ## Combinators
//!
#![cfg_attr(feature = "enigo", doc = "```")]
#![cfg_attr(not(feature = "enigo"), doc = "```ignore")]
//! use kemuler::prelude::*;
//! use kemuler::simulators::{Enigo, EnigoKeyExt};
//! use enigo::{Key, MouseButton};
//! use kemuler::combinator::Sleep;
//!
//! let mut enigo = Enigo::new();
//!
//! Key::Alt.down()
//!     .then(Key::Tab.down())
//!     .then(Key::Alt.up())
//!     .then(Key::Tab.up())
//!     .run_with(&mut enigo);
//!
//! // Tuple supports! (up to 64 indexes)
//! // If you some how need much more than that then nested tuple will suffice.
//! (
//!     Key::Control.down(),
//!     MouseButton::Right.down(),
//!     Sleep::from_ms(1000),
//!     Key::Control.up(),
//!     MouseButton::Right.up(),
//! ).run_with(&mut enigo);
//!
//! // And array!
//! [
//!     Key::Alt.down(),
//!     Key::Tab.down(),
//!     Key::Alt.up(),
//!     Key::Tab.up()
//! ].run_with(&mut enigo);
//!
//! // Other useful combinators!
//! (MouseButton::Left.click(), Key::Space.click().sleep_ms(500))
//!     .repeat(20)
//!     .run_with(&mut enigo);
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
