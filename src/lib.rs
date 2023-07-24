//! # Kemuler
//!
//! Welcome to an experimental input simulation crate, `kemuler`!
//!
//! This crate offers a different kind of input simulator framework,
//! utilizing Rust’s type system to its full potential (or just over-engineered)
//! instead of the usual key_down(Key).
//! Why?
//! Combinators!
//! It is experimental, so
//! that’s currently a separate branch on GitHub.
//! But the crate is still posted because of current usable features.
//!
//! Current features:
//! - Multiple backends support (called `Simulator` in here).
//!   Built-ins:
//!   - Enigo (The crate that helped me make this crate and it is cross-platform.)
//!   - WinDirect (support for Window's DirectX/DirectInput something)
//!
//! Some drawbacks:
//! - Bunch of boilerplate is currently needed. (working on it)
//!
//! # Examples
//! ## Basic
//!
//! ```
//! use kemuler::prelude::*;
//!
//! // Requires the feature "enigo"
//! use kemuler::simulator::Enigo;
//!
//! // method 1
//! Key::A.down().simulate_with(&Enigo);
//!
//! // method 2
//! // (the usual way is still supported)
//! Enigo.simulate_input(Key::A.down())

pub mod combinator;
pub mod input;
pub mod simulate;

pub mod inputs;
pub mod simulators;
