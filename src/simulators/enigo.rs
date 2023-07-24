//! Simulate input using `Enigo`.

use std::fmt;

#[cfg(feature = "event-log")]
use crate::simulators::EventLog;
use crate::{
    event::{ChangeBy, SetTo},
    inputs::common,
    simulate::{Simulate, TrySimulate},
};
use enigo::{KeyboardControllable, MouseControllable};

/// Simulate input using `Enigo`.
///
/// Implemented events:
/// - `SetTo   <enigo::Key           , bool>`
/// - `SetTo   <enigo::MouseButton   , bool>`
/// - `SetTo   <common::MouseButton  , bool>`
/// - `SetTo   <common::MousePosition, (i32, i32)>`
/// - `ChangeBy<common::MousePosition, (i32, i32)>`
/// - `ChangeBy<common::MouseScroll  , (i32, i32)>`
#[derive(Debug, Default)]
pub struct Enigo(enigo::Enigo);

impl Enigo {
    pub fn new() -> Enigo {
        Enigo(enigo::Enigo::new())
    }

    pub fn from_inner(enigo: enigo::Enigo) -> Enigo {
        Enigo(enigo)
    }

    pub fn into_inner(self) -> enigo::Enigo {
        self.0
    }

    pub fn inner(&self) -> &enigo::Enigo {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut enigo::Enigo {
        &mut self.0
    }
}

fn enigoify_common_mouse_button(button: common::MouseButton) -> enigo::MouseButton {
    match button {
        common::MouseButton::Left => enigo::MouseButton::Left,
        common::MouseButton::Middle => enigo::MouseButton::Middle,
        common::MouseButton::Right => enigo::MouseButton::Right,
    }
}

impl Simulate<SetTo<enigo::Key, bool>> for Enigo {
    fn run(&mut self, event: SetTo<enigo::Key, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = event;
        if is_down {
            self.0.key_down(key)
        } else {
            self.0.key_up(key)
        }
    }
}
impl Simulate<SetTo<enigo::MouseButton, bool>> for Enigo {
    fn run(&mut self, event: SetTo<enigo::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = event;
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulate<SetTo<common::MouseButton, bool>> for Enigo {
    fn run(&mut self, event: SetTo<common::MouseButton, bool>) {
        let SetTo { input, to: is_down } = event;
        let button = enigoify_common_mouse_button(input);
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulate<SetTo<common::MousePosition, (i32, i32)>> for Enigo {
    fn run(&mut self, event: SetTo<common::MousePosition, (i32, i32)>) {
        let SetTo { input: _, to } = event;
        self.0.mouse_move_to(to.0, to.1)
    }
}

impl Simulate<ChangeBy<common::MousePosition, (i32, i32)>> for Enigo {
    fn run(&mut self, event: ChangeBy<common::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = event;
        self.0.mouse_move_relative(by.0, by.1)
    }
}

impl Simulate<ChangeBy<common::MouseScroll, (i32, i32)>> for Enigo {
    fn run(&mut self, event: ChangeBy<common::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = event;
        if by.0 != 0 {
            self.0.mouse_scroll_x(by.0);
        }
        if by.1 != 0 {
            self.0.mouse_scroll_x(by.1);
        }
    }
}

#[derive(Debug)]
pub struct UnknownEventError;

impl fmt::Display for UnknownEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown event")
    }
}

impl std::error::Error for UnknownEventError {}

#[cfg(feature = "event-log")]
impl TrySimulate<Box<dyn EventLog>> for Enigo {
    type Error = UnknownEventError;

    fn try_run(
        &mut self,
        event: Box<dyn EventLog>,
    ) -> Result<(), (Self::Error, Box<dyn EventLog>)> {
        macro_rules! simulate {
            ($($ty:ty)*) => {
                $(
                    if let Some(v) = event.as_any().downcast_ref::<$ty>() {
                        self.run(v.clone());
                        return Ok(());
                    }
                )*
            };
        }
        simulate! {
            SetTo<enigo::Key, bool>
            SetTo<enigo::MouseButton, bool>
            SetTo<common::MouseButton, bool>
            SetTo<common::MousePosition, (i32, i32)>
            ChangeBy<common::MousePosition, (i32, i32)>
            ChangeBy<common::MouseScroll, (i32, i32)>
        }

        Err((UnknownEventError, event))
    }
}
