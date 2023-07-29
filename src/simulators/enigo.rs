//! Simulate input using `Enigo`.

use crate::{
    combinator::{Combine, Sequence},
    input_event::*,
    inputs::common,
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
    fn click(self) -> Sequence<(SetTo<Self, bool>, SetTo<Self, bool>)> {
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

fn enigoify_common_keyboard(keyboard: common::Key) -> enigo::Key {
    match keyboard {
        common::Key::Alt => enigo::Key::Alt,
        common::Key::Shift => enigo::Key::Shift,
        common::Key::Control => enigo::Key::Control,
        common::Key::F1 => enigo::Key::F1,
        common::Key::F2 => enigo::Key::F2,
        common::Key::F3 => enigo::Key::F3,
        common::Key::F4 => enigo::Key::F4,
        common::Key::F5 => enigo::Key::F5,
        common::Key::F6 => enigo::Key::F6,
        common::Key::F7 => enigo::Key::F7,
        common::Key::F8 => enigo::Key::F8,
        common::Key::F9 => enigo::Key::F9,
        common::Key::F10 => enigo::Key::F10,
        common::Key::F11 => enigo::Key::F11,
        common::Key::F12 => enigo::Key::F12,
        common::Key::CapsLock => enigo::Key::CapsLock,
        common::Key::End => enigo::Key::End,
        common::Key::Home => enigo::Key::Home,
        common::Key::PageUp => enigo::Key::PageUp,
        common::Key::PageDown => enigo::Key::PageDown,
        common::Key::Escape => enigo::Key::Escape,
        common::Key::Return => enigo::Key::Return,
        common::Key::Space => enigo::Key::Space,
        common::Key::Tab => enigo::Key::Tab,
        common::Key::Backspace => enigo::Key::Backspace,
        common::Key::Delete => enigo::Key::Delete,
        common::Key::UpArrow => enigo::Key::UpArrow,
        common::Key::DownArrow => enigo::Key::DownArrow,
        common::Key::LeftArrow => enigo::Key::LeftArrow,
        common::Key::RightArrow => enigo::Key::RightArrow,
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

impl Simulate<SetTo<common::Key, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common::Key, bool>) {
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
