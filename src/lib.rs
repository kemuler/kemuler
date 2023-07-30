#![doc = include_str!("../README.md")]
//! # Examples
//! All exmaples are using a private simulator for demonstration purposes.
//! `assert_event` is not available in public API.
//!
//! ## The basic
//!
//! ```
//! # use kemuler::simulators::string_event_logger::StringEventLogger as Simulator;
//! # use kemuler::assert_event;
//! use kemuler::prelude::*;
//!
//! let mut s = Simulator::new(); // use your preferred simulator
//!
//! Key::Shift.down().run_with(&mut s);
//! Key::Shift.up().run_with(&mut s);
//! Char('w').down().run_with(&mut s);
//! Char('w').up().run_with(&mut s);
//!
//! assert_event!(s, 0, Key::Shift.down());
//! assert_event!(s, 1, Key::Shift.up());
//! assert_event!(s, 2, Char('w').down());
//! assert_event!(s, 3, Char('w').up());
//! ```
//!
//! ## Combinators Overview
//!
//! `.then` method chains simulatable together to run one by one.
//! ```
//! # use kemuler::simulators::string_event_logger::StringEventLogger as Simulator;
//! # use kemuler::assert_event;
//! use kemuler::prelude::*;
//! use kemuler::combinator::Sleep;
//!
//! // use your preferred simulator—you probably got the gist by now.
//! let mut s = Simulator::new();
//!
//! Key::Alt.down()
//!     .then(Key::Tab.down())
//!     .then(Key::Alt.up())
//!     .then(Key::Tab.up())
//!     .run_with(&mut s);
//!
//! assert_event!(s, 0, Key::Alt.down());
//! assert_event!(s, 1, Key::Tab.down());
//! assert_event!(s, 2, Key::Alt.up());
//! assert_event!(s, 3, Key::Tab.up());
//! assert_eq!(s.data.len(), 4); // only 4 events has been passed to the simulator
//! ```
//!
//! Tuple supports! Only up to 32 indexes;
//! if you some how need much more than that then nested tuple will suffice.
//! Chains simulatable one by one without `.then`.
//! (`then` actually returns this)
//! ```
//! # use kemuler::{
//! #     simulators::string_event_logger::StringEventLogger as Simulator,
//! #     assert_event, prelude::*, combinator::Sleep
//! # };
//! # let mut s = Simulator::new();
//! (
//!     Key::Control.down(),
//!     MouseButton::Right.down(),
//!     Sleep::from_ms(100),
//!     Key::Control.up(),
//!     MouseButton::Right.up(),
//! )
//!     // wrap this tuple with the `Sequence` type to make it simulatable
//!     .seq()
//!     .run_with(&mut s);
//!
//! assert_event!(s, 0, Key::Control.down());
//! assert_event!(s, 1, MouseButton::Right.down());
//! assert_event!(s, 2, Key::Control.up());
//! assert_event!(s, 3, MouseButton::Right.up());
//! assert_eq!(s.data.len(), 4); // only 4 events has been passed to the simulator
//! // sleep event is not logged but it should work :P
//! ```
//!
//! Iterator supports!
//! This does a for loop internally and simulate each item
//! as long as the item is simulatable.
//! ```
//! # use kemuler::{
//! #     simulators::string_event_logger::StringEventLogger as Simulator,
//! #     assert_event, prelude::*, combinator::Sleep
//! # };
//! # let mut s = Simulator::new();
//! [
//!     Key::Alt.down(),
//!     Key::Tab.down(),
//!     Key::Alt.up(),
//!     Key::Tab.up()
//! ]
//!     // wrap this iterator with the `IterSequence` type
//!     // to make this iterator simulatable.
//!     // This supports every type that implements `IntoIterator`.
//!     .iter_seq()
//!     .run_with(&mut s);
//!
//! assert_event!(s, 0, Key::Alt.down());
//! assert_event!(s, 1, Key::Tab.down());
//! assert_event!(s, 2, Key::Alt.up());
//! assert_event!(s, 3, Key::Tab.up());
//! assert_eq!(s.data.len(), 4); // only 4 events has been passed to the simulator
//! ```
//!
//! Other useful combinators!
//! ```
//! # use kemuler::{
//! #     simulators::string_event_logger::StringEventLogger as Simulator,
//! #     assert_event, prelude::*
//! # };
//! # let mut s = Simulator::new();
//! // do these 3 times:
//! //   left click
//! //   space bar click
//! //   wait 10 millisecond
//! //
//! (
//!     MouseButton::Left.click(),
//!     Key::Space.click().sleep_ms(10),
//! )
//!     .seq()
//!     .repeat(2)
//!     .run_with(&mut s);
//!
//! assert_event!(s, 0, MouseButton::Left.down());
//! assert_event!(s, 1, MouseButton::Left.up());
//! assert_event!(s, 2, Key::Space.down());
//! assert_event!(s, 3, Key::Space.up());
//! assert_event!(s, 4, MouseButton::Left.down());
//! assert_event!(s, 5, MouseButton::Left.up());
//! assert_event!(s, 6, Key::Space.down());
//! assert_event!(s, 7, Key::Space.up());
//! assert_eq!(s.data.len(), 8); // only 8 events has been passed to the simulator
//! // sleep event is not logged but it should work :P
//! ```
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod combinator;
pub mod input_event;
pub mod simulatable;
pub mod simulator;

pub mod common_inputs;
pub mod simulators;

pub mod prelude {
    //! Re-exports

    use super::*;

    pub use combinator::Combine;
    pub use common_inputs::*;
    pub use simulatable::Simulatable;
}
