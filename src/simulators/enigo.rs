//! Simulate input using `Enigo`.

use crate::{
    event::{ChangeBy, SetTo},
    inputs::common,
    simulate::Simulator,
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
}

fn enigoify_common_mouse_button(button: common::MouseButton) -> enigo::MouseButton {
    match button {
        common::MouseButton::Left => enigo::MouseButton::Left,
        common::MouseButton::Middle => enigo::MouseButton::Middle,
        common::MouseButton::Right => enigo::MouseButton::Right,
    }
}

impl Simulator<SetTo<enigo::Key, bool>> for Enigo {
    fn run(&mut self, input: SetTo<enigo::Key, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = input;
        if is_down {
            self.0.key_down(key)
        } else {
            self.0.key_up(key)
        }
    }
}
impl Simulator<SetTo<enigo::MouseButton, bool>> for Enigo {
    fn run(&mut self, input: SetTo<enigo::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = input;
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulator<SetTo<common::MouseButton, bool>> for Enigo {
    fn run(&mut self, input: SetTo<common::MouseButton, bool>) {
        let SetTo { input, to: is_down } = input;
        let button = enigoify_common_mouse_button(input);
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulator<SetTo<common::MousePosition, (i32, i32)>> for Enigo {
    fn run(&mut self, input: SetTo<common::MousePosition, (i32, i32)>) {
        let SetTo { input: _, to } = input;
        self.0.mouse_move_to(to.0, to.1)
    }
}

impl Simulator<ChangeBy<common::MousePosition, (i32, i32)>> for Enigo {
    fn run(&mut self, input: ChangeBy<common::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = input;
        self.0.mouse_move_relative(by.0, by.1)
    }
}

impl Simulator<ChangeBy<common::MouseScroll, (i32, i32)>> for Enigo {
    fn run(&mut self, input: ChangeBy<common::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = input;
        if by.0 != 0 {
            self.0.mouse_scroll_x(by.0);
        }
        if by.1 != 0 {
            self.0.mouse_scroll_x(by.1);
        }
    }
}
