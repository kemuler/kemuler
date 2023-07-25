#![doc = include_str!("../README.md")]
//! # Examples
//! All these examples requires the feature "enigo".
//!
//! ## Basic
//!
#![cfg_attr(feature = "enigo", doc = "```")]
#![cfg_attr(not(feature = "enigo"), doc = "```ignore")]
//! use kemuler::prelude::*;
//! use kemuler::simulators::enigo::{enigo, Enigo, EnigoKeyExt};
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
//! use kemuler::simulators::enigo::{enigo, Enigo, EnigoKeyExt};
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
//! // Tuple supports! (up to 32 indexes)
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
//!
//! // do these 20 times:
//! //   left click
//! //   space bar click
//! //   wait 500 millisecond
//! //
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

// #[test]
// fn test() {

//     use crate::combinator::IntoSimulatableIter;
//     use crate as kemuler;
//     use enigo::{Key, MouseButton};
//     use kemuler::combinator::Sleep;
//     use kemuler::prelude::*;
//     use kemuler::simulators::enigo::{Enigo, EnigoKeyExt};

//     let mut enigo = Enigo::new();

//     let a = Keyboard::Alt
//         .down()
//         .then(Keyboard::Tab.down())
//         .then(Keyboard::Alt.up())
//         .then(Keyboard::Tab.up());
//     println!("{a}");
//     return;
//     a.run_with(&mut enigo);

//     (
//         Key::Control.down(),
//         MouseButton::Right.down(),
//         Sleep::from_ms(1000),
//         Key::Control.up(),
//         MouseButton::Right.up(),
//     )
//         .run_with(&mut enigo);

//     [
//         Key::Alt.down(),
//         Key::Tab.down(),
//         Key::Alt.up(),
//         Key::Tab.up(),
//     ]
//     .into_simulatable_iter()
//     .run_with(&mut enigo);

//     (MouseButton::Left.click(), Key::Space.click().sleep_ms(500))
//         .repeat(20)
//         .run_with(&mut enigo);
// }
