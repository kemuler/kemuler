//! Simulate input using `Enigo`.

use crate::{common_inputs, input_event::*, simulator::Simulate};
use common_inputs::ButtonLike;
use enigo::{KeyboardControllable, MouseControllable};

pub use enigo;

impl ButtonLike for enigo::Key {}
impl ButtonLike for enigo::MouseButton {}

/// Simulate input using `Enigo`.
///
/// Implemented simulatables:
/// - `SetTo   <enigo::Key                  , bool>`
/// - `SetTo   <enigo::MouseButton          , bool>`
/// - `SetTo   <common_inputs::Key          , bool>`
/// - `SetTo   <common_inputs::MouseButton  , bool>`
/// - `SetTo   <common_inputs::MousePosition, (i32, i32)>`
/// - `ChangeBy<common_inputs::MousePosition, (i32, i32)>`
/// - `ChangeBy<common_inputs::MouseScroll  , (i32, i32)>`
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

fn enigoify_common_mouse_button(button: common_inputs::MouseButton) -> enigo::MouseButton {
    match button {
        common_inputs::MouseButton::Left => enigo::MouseButton::Left,
        common_inputs::MouseButton::Middle => enigo::MouseButton::Middle,
        common_inputs::MouseButton::Right => enigo::MouseButton::Right,
    }
}

fn enigoify_common_key(key: common_inputs::Key) -> enigo::Key {
    match key {
        common_inputs::Key::Alt => enigo::Key::Alt,
        common_inputs::Key::Shift => enigo::Key::Shift,
        common_inputs::Key::Control => enigo::Key::Control,
        common_inputs::Key::F1 => enigo::Key::F1,
        common_inputs::Key::F2 => enigo::Key::F2,
        common_inputs::Key::F3 => enigo::Key::F3,
        common_inputs::Key::F4 => enigo::Key::F4,
        common_inputs::Key::F5 => enigo::Key::F5,
        common_inputs::Key::F6 => enigo::Key::F6,
        common_inputs::Key::F7 => enigo::Key::F7,
        common_inputs::Key::F8 => enigo::Key::F8,
        common_inputs::Key::F9 => enigo::Key::F9,
        common_inputs::Key::F10 => enigo::Key::F10,
        common_inputs::Key::F11 => enigo::Key::F11,
        common_inputs::Key::F12 => enigo::Key::F12,
        common_inputs::Key::CapsLock => enigo::Key::CapsLock,
        common_inputs::Key::End => enigo::Key::End,
        common_inputs::Key::Home => enigo::Key::Home,
        common_inputs::Key::PageUp => enigo::Key::PageUp,
        common_inputs::Key::PageDown => enigo::Key::PageDown,
        common_inputs::Key::Escape => enigo::Key::Escape,
        common_inputs::Key::Enter => enigo::Key::Return,
        common_inputs::Key::Space => enigo::Key::Space,
        common_inputs::Key::Tab => enigo::Key::Tab,
        common_inputs::Key::Backspace => enigo::Key::Backspace,
        common_inputs::Key::Delete => enigo::Key::Delete,
        common_inputs::Key::UpArrow => enigo::Key::UpArrow,
        common_inputs::Key::DownArrow => enigo::Key::DownArrow,
        common_inputs::Key::LeftArrow => enigo::Key::LeftArrow,
        common_inputs::Key::RightArrow => enigo::Key::RightArrow,
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

impl Simulate<SetTo<common_inputs::Key, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::Key, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = simulatable;
        let key = enigoify_common_key(key);
        if is_down {
            self.0.key_down(key)
        } else {
            self.0.key_up(key)
        }
    }
}

impl Simulate<SetTo<common_inputs::Char, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::Char, bool>) {
        let SetTo {
            input: char_key,
            to: is_down,
        } = simulatable;
        if is_down {
            self.0.key_down(enigo::Key::Layout(char_key.0))
        } else {
            self.0.key_up(enigo::Key::Layout(char_key.0))
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

impl Simulate<SetTo<common_inputs::MouseButton, bool>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MouseButton, bool>) {
        let SetTo { input, to: is_down } = simulatable;
        let button = enigoify_common_mouse_button(input);
        if is_down {
            self.0.mouse_down(button)
        } else {
            self.0.mouse_up(button)
        }
    }
}

impl Simulate<SetTo<common_inputs::MousePosition, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MousePosition, (i32, i32)>) {
        let SetTo { input: _, to } = simulatable;
        self.0.mouse_move_to(to.0, to.1)
    }
}

impl Simulate<ChangeBy<common_inputs::MousePosition, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        self.0.mouse_move_relative(by.0, by.1)
    }
}

impl Simulate<ChangeBy<common_inputs::MouseScroll, (i32, i32)>> for Enigo {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        if by.0 != 0 {
            self.0.mouse_scroll_x(by.0);
        }
        if by.1 != 0 {
            self.0.mouse_scroll_x(by.1);
        }
    }
}

impl<'a> Simulate<Execute<common_inputs::StrSequence<'a>>> for Enigo {
    fn simulate(&mut self, simulatable: Execute<common_inputs::StrSequence>) {
        let Execute { input } = simulatable;
        self.0.key_sequence(input.0)
    }
}
