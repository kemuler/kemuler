//! Simulate input using `Enigo`.

use crate::{
    combinator::{AndThen, Combine},
    inputs::common,
    simulatable::{ChangeBy, SetTo},
    simulator::Simulate,
};
use enigo::{KeyboardControllable, MouseControllable};

pub use enigo;

pub trait EnigoKeyExt: Sized + Clone {
    /// Set the key.
    /// This is a convenience shorthand for
    /// ```
    /// SetTo { input: self, to: to }
    /// ```
    fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the key.
    /// This is a convenience shorthand for
    /// ```
    /// SetTo { input: self, to: true }
    /// ```
    fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the button.
    /// Release the key
    /// This is a convenience shorthand for
    /// ```
    /// SetTo { input: self, to: false }
    /// ```
    fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the key.
    /// This is a convenience shorthand for
    /// ```
    /// AndThen(
    ///     SetTo { input: self, to: true },
    ///     SetTo { input: self, to: false }
    /// )
    /// ```
    fn click(self) -> AndThen<SetTo<Self, bool>, SetTo<Self, bool>> {
        self.clone().down().then(self.up())
    }
}

impl EnigoKeyExt for enigo::Key {}
impl EnigoKeyExt for enigo::MouseButton {}

/// Simulate input using `Enigo`.
///
/// Implemented simulatables:
/// - `SetTo   <enigo::Key           , bool>`
/// - `SetTo   <common::Keyboard     , bool>`
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

fn enigoify_common_keyboard(keyboard: common::Keyboard) -> enigo::Key {
    match keyboard {
        common::Keyboard::Alt => enigo::Key::Alt,
        // common::Keyboard::LAlt => todo!(),
        // common::Keyboard::RAlt => todo!(),
        common::Keyboard::Shift => enigo::Key::Shift,
        common::Keyboard::LShift => enigo::Key::LShift,
        common::Keyboard::RShift => enigo::Key::RShift,
        common::Keyboard::Control => enigo::Key::Control,
        common::Keyboard::LControl => enigo::Key::LControl,
        common::Keyboard::RControl => enigo::Key::RControl,
        common::Keyboard::F1 => enigo::Key::F1,
        common::Keyboard::F2 => enigo::Key::F2,
        common::Keyboard::F3 => enigo::Key::F3,
        common::Keyboard::F4 => enigo::Key::F4,
        common::Keyboard::F5 => enigo::Key::F5,
        common::Keyboard::F6 => enigo::Key::F6,
        common::Keyboard::F7 => enigo::Key::F7,
        common::Keyboard::F8 => enigo::Key::F8,
        common::Keyboard::F9 => enigo::Key::F9,
        common::Keyboard::F10 => enigo::Key::F10,
        common::Keyboard::F11 => enigo::Key::F11,
        common::Keyboard::F12 => enigo::Key::F12,
        common::Keyboard::CapsLock => enigo::Key::CapsLock,
        common::Keyboard::End => enigo::Key::End,
        common::Keyboard::Home => enigo::Key::Home,
        common::Keyboard::PageUp => enigo::Key::PageUp,
        common::Keyboard::PageDown => enigo::Key::PageDown,
        common::Keyboard::Escape => enigo::Key::Escape,
        common::Keyboard::Return => enigo::Key::Return,
        common::Keyboard::Space => enigo::Key::Space,
        common::Keyboard::Tab => enigo::Key::Tab,
        common::Keyboard::Backspace => enigo::Key::Backspace,
        common::Keyboard::Delete => enigo::Key::Delete,
        common::Keyboard::UpArrow => enigo::Key::UpArrow,
        common::Keyboard::DownArrow => enigo::Key::DownArrow,
        common::Keyboard::LeftArrow => enigo::Key::LeftArrow,
        common::Keyboard::RightArrow => enigo::Key::RightArrow,
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

impl Simulate<SetTo<common::Keyboard, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common::Keyboard, bool>) {
        let SetTo {
            input: keyboard,
            to: is_down,
        } = simulatable;
        let key = enigoify_common_keyboard(keyboard);
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
