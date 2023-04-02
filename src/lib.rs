pub mod combinator;
pub mod emulatable;
pub mod peripherals;
pub mod timing;
pub mod prelude {
    use super::*;

    pub use combinator::Combine;
    pub use emulatable::{EmulateAbsoluteValue, EmulateBinaryState, EmulateRelativeValue};
    pub use peripherals::{Keyboard, MouseButton, MousePosition, MouseScroll};
    pub use timing::{IntoDuration, Wait};
}
