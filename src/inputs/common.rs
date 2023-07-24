//! Generally found/a standard input

use crate::{combinator::*, event::*};
use std::fmt;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyboard {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,

    /// alt key on Linux and Windows (option key on macOS)
    Alt, LAlt, RAlt,
    Shift, LShift, RShift,
    Control, LControl, RControl,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    CapsLock,

    End, Home, PageUp, PageDown,

    Escape, Return, Space, Tab,

    Backspace, Delete,

    UpArrow, DownArrow, LeftArrow, RightArrow,
}

impl Keyboard {
    /// Set this key
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the key.
    /// This is a convenience shorthand, see code for more details.
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the key
    /// This is a convenience shorthand, see code for more details.
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MousePosition;

impl MousePosition {
    pub fn move_to(self, x: i32, y: i32) -> SetTo<Self, (i32, i32)> {
        SetTo {
            input: self,
            to: (x, y),
        }
    }

    pub fn move_by(self, x: i32, y: i32) -> ChangeBy<Self, (i32, i32)> {
        ChangeBy {
            input: self,
            by: (x, y),
        }
    }
}

impl fmt::Display for MousePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseScroll;

impl MouseScroll {
    /// Scroll mouse wheel.
    pub fn scroll_by(self, x: i32, y: i32) -> ChangeBy<Self, (i32, i32)> {
        ChangeBy {
            input: self,
            by: (x, y),
        }
    }
}

impl fmt::Display for MouseScroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    /// Set this button.
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the button.
    /// This is a convenience shorthand, see code for more details.
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the button.
    /// This is a convenience shorthand, see code for more details.
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the button.
    /// This is a convenience shorthand, see code for more details.
    pub fn click(self) -> AndThen<SetTo<Self, bool>, SetTo<Self, bool>> {
        self.down().then(self.up())
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
