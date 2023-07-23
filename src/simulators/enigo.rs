//! Simulate input using `Enigo`.

use crate::{
    input::{ChangeBy, SetTo},
    inputs::common,
    simulate::Simulator,
};
use enigo::{KeyboardControllable, MouseControllable};

/// Simulate input using `Enigo`.
#[derive(Debug)]
pub struct Enigo(enigo::Enigo);

fn enigoify_common_mouse_button(button: common::MouseButton) -> enigo::MouseButton {
    match button {
        common::MouseButton::Left => enigo::MouseButton::Left,
        common::MouseButton::Middle => enigo::MouseButton::Middle,
        common::MouseButton::Right => enigo::MouseButton::Right,
        common::MouseButton::X1 => enigo::MouseButton::Forward,
        common::MouseButton::X2 => enigo::MouseButton::Back,
    }
}

impl Simulator<SetTo<enigo::Key, bool>> for Enigo {
    fn run(&mut self, input: SetTo<enigo::Key, bool>) {
        let SetTo { input, to: is_down } = input;
        if is_down {
            self.0.key_down(input)
        } else {
            self.0.key_up(input)
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
