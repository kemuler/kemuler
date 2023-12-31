#![doc = include_str!("../README.md")]
//! # Examples
//!
//! ## The basic
//!
//! ```
//! # use kemuler::string_event_logger::StringEventLogger as Simulator;
//! # use kemuler::assert_events;
//! use kemuler::prelude::*;
//!
//! let mut s = Simulator::new(); // use your preferred simulator
//!
//! Key::Shift.down().run_with(&mut s);
//! Key::Shift.up().run_with(&mut s);
//! Char('w').down().run_with(&mut s);
//! Char('w').up().run_with(&mut s);
//!
#![doc = doc_events!(
    4;
    "Key::Shift.down()"
    "Key::Shift.up()"
    "Char('w').down()"
    "Char('w').up()"    
)]
//! ```
//!
//! ## Combinators Overview
//!
//! `.then` method chains simulatable together to run one by one.
//! ```
//! # use kemuler::string_event_logger::StringEventLogger as Simulator;
//! # use kemuler::assert_events;
//! use kemuler::prelude::*;
//!
//! // use your preferred simul—ahh you got it.
//! let mut s = Simulator::new();
//!
//! Key::Alt.down()
//!     .then(Key::Tab.down())
//!     .then(Key::Alt.up())
//!     .then(Key::Tab.up())
//!     .run_with(&mut s);
//!
#![doc = doc_events!(
    4;
    "Key::Alt.down()"
    "Key::Tab.down()"
    "Key::Alt.up()"
    "Key::Tab.up()"    
)]
//! ```
//!
//! Tuple supports! (Up to 32 indexes; nest tuple if you need more)
//! Chain simulatables one by one starting from `.0`.
//! ```
//! # use kemuler::{
//! #     string_event_logger::StringEventLogger as Simulator,
//! #     assert_events, prelude::*
//! # };
//! # let mut s = Simulator::new();
//! use kemuler::utils::time::millis;
//!
//! (
//!     Key::Control.down(),
//!     MouseButton::Right.down(),
//!     Sleep(millis(100)),
//!     Key::Control.up(),
//!     MouseButton::Right.up(),
//! )
//!     // Wrap this tuple with the `SimTuple`
//!     .sim_tuple()
//!     .run_with(&mut s);
//!
#![doc = doc_events!(
    4;
    "Key::Control.down()"
    "MouseButton::Right.down()"
    "Key::Control.up()"
    "MouseButton::Right.up()"    
)]
//! ```
//!
//! Iterator supports!
//! This does a for loop internally and simulate each item.
//! This supports every type that implements `IntoIterator` and item is `Simulatable`.
//! ```
//! # use kemuler::{
//! #     string_event_logger::StringEventLogger as Simulator,
//! #     assert_events, prelude::*
//! # };
//! # let mut s = Simulator::new();
//! [
//!     Key::Alt.down(),
//!     Key::Tab.down(),
//!     Key::Alt.up(),
//!     Key::Tab.up()
//! ]
//!     // Wrap this iterator with the `SimIter` type.
//!     .sim_iter()
//!     .run_with(&mut s);
//!
#![doc = doc_events!(
    4;
    "Key::Alt.down()"
    "Key::Tab.down()"
    "Key::Alt.up()"
    "Key::Tab.up()"    
)]
//! ```
//!
//! Other useful combinators!
//! ```
//! # use kemuler::{
//! #     string_event_logger::StringEventLogger as Simulator,
//! #     assert_events, prelude::*
//! # };
//! # let mut s = Simulator::new();
//! use kemuler::utils::time::millis;
//!
//! // do these 3 times:
//! //   left click
//! //   space bar click
//! //   wait 10 millisecond
//! //
//! (
//!     MouseButton::Left.click(),
//!     Key::Space.click().sleep(millis(10)),
//! )
//!     .sim_tuple()
//!     .repeat(3)
//!     .run_with(&mut s);
//!
#![doc = doc_events!(
    8;
    "MouseButton::Left.down()"
    "MouseButton::Left.up()"
    "Key::Space.down()"
    "Key::Space.up()"
    "MouseButton::Left.down()"
    "MouseButton::Left.up()"
    "Key::Space.down()"
    "Key::Space.up()"
)]
//! ```
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

macro_rules! doc_event {
    ($event:literal) => {
        concat!(
            "// ", $event, ";", // visible to user line
            "\n# ", $event, "," // invisible to user line
        )
    };
}

macro_rules! doc_events {
    ($len:literal; $($events:literal)+) => {
        concat!(
            "// Inputs that has been passed to the simulator\n",
            "// (A world without combinator):\n",
            "# assert_events!(\n",
            "#     s, 0,\n",
            $(doc_event!($events), "\n",)+
            "# );\n",
            "# assert_eq!(s.data.len(), ",
            $len,
            ");",
        )
    };
}
pub mod combinator;
pub mod input_event;
pub mod simulatable;
pub mod simulator;

pub mod common_inputs;
// this is pub because it only exist in test so no user can see it
// (unless feature = "test")
// (reminder for my dumbass)
#[cfg(any(test, doctest, feature = "test"))]
#[macro_use]
pub mod string_event_logger;
pub mod utils;

pub mod prelude {
    //! Re-exports

    use super::*;

    #[cfg(feature = "spin_sleep")]
    pub use combinator::SpinSleep;
    pub use combinator::{Combine, Sleep};
    pub use common_inputs::*;
    pub use simulatable::Simulatable;
}
