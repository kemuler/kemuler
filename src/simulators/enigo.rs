//! Simulate input using `Enigo`.

use crate::{
    inputs::common,
    simulatable::{ChangeBy, SetTo},
    simulator::Simulate,
};
use enigo::{KeyboardControllable, MouseControllable};

/// Simulate input using `Enigo`.
///
/// Implemented simulatables:
/// - `SetTo   <enigo::Key           , bool>`
/// - `SetTo   <enigo::MouseButton   , bool>`
/// - `SetTo   <common::MouseButton  , bool>`
/// - `SetTo   <common::MousePosition, (i32, i32)>`
/// - `ChangeBy<common::MousePosition, (i32, i32)>`
/// - `ChangeBy<common::MouseScroll  , (i32, i32)>`
#[derive(Debug, Default)]
pub struct Enigo(pub enigo::Enigo);

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
    fn simulate(&mut self, simulatable: SetTo<enigo::Key, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = simulatable;
        if is_down {
            self.0.key_down(key)
        } else {
            self.0.key_up(key)
        }
    }
}
impl Simulate<SetTo<enigo::MouseButton, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<enigo::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = simulatable;
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulate<SetTo<common::MouseButton, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common::MouseButton, bool>) {
        let SetTo { input, to: is_down } = simulatable;
        let button = enigoify_common_mouse_button(input);
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulate<SetTo<common::MousePosition, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common::MousePosition, (i32, i32)>) {
        let SetTo { input: _, to } = simulatable;
        self.0.mouse_move_to(to.0, to.1)
    }
}

impl Simulate<ChangeBy<common::MousePosition, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: ChangeBy<common::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        self.0.mouse_move_relative(by.0, by.1)
    }
}

impl Simulate<ChangeBy<common::MouseScroll, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: ChangeBy<common::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        if by.0 != 0 {
            self.0.mouse_scroll_x(by.0);
        }
        if by.1 != 0 {
            self.0.mouse_scroll_x(by.1);
        }
    }
}
