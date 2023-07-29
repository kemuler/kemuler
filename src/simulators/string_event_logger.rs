use std::fmt;

use crate::{common_inputs, input_event::*, simulator::Simulate};

/// Mock simulator to be used with testing.
///
/// Implemented simulatables:
/// - `SetTo   <enigo::Key                  , bool>`
/// - `SetTo   <enigo::MouseButton          , bool>`
/// - `SetTo   <common_inputs::Key          , bool>`
/// - `SetTo   <common_inputs::MouseButton  , bool>`
/// - `SetTo   <common_inputs::MousePosition, (i32, i32)>`
/// - `ChangeBy<common_inputs::MousePosition, (i32, i32)>`
/// - `ChangeBy<common_inputs::MouseScroll  , (i32, i32)>`
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct StringEventLogger {
    data: Vec<String>,
}

impl StringEventLogger {
    pub fn new() -> StringEventLogger {
        StringEventLogger::default()
    }

    pub fn push<T: fmt::Debug>(&mut self, t: &T) {
        self.data.push(format!("{:?}", t));
    }
}

macro_rules! impl_logger {
    ($( $(#[$attr:meta])* $ty:ty)*) => {
        $(
            $(#[$attr])*
            impl Simulate<$ty> for StringEventLogger {
                fn simulate(&mut self, simulatable: $ty) {
                    self.push(&simulatable)
                }
            }
        )*
    };
}

impl_logger! {
    #[cfg(feature = "enigo")] SetTo<enigo::Key, bool>
    #[cfg(feature = "enigo")] SetTo<enigo::MouseButton, bool>
    SetTo<common_inputs::Key, bool>
    SetTo<common_inputs::MouseButton, bool>
    SetTo<common_inputs::MousePosition, (i32, i32)>
    ChangeBy<common_inputs::MousePosition, (i32, i32)>
    ChangeBy<common_inputs::MouseScroll, (i32, i32)>
}
