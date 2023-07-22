//! Generally found/a standard input

use crate::input::*;
use std::fmt;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
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

impl Key {
    /// Toggle this key
    pub fn toggle(self) -> Toggle<Self> {
        Toggle { input: self }
    }

    /// Set this key
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the key (`set_to(true)`)
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the key (`set_to(false)`)
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }
}

impl Input for Key {}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Input for MousePosition {}

impl fmt::Display for MousePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MouseScroll;

impl MouseScroll {
    pub fn scroll_by(self, x: i32, y: i32) -> ChangeBy<Self, (i32, i32)> {
        ChangeBy {
            input: self,
            by: (x, y),
        }
    }
}

impl Input for MouseScroll {}

impl fmt::Display for MouseScroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseButton {
    /// Toggle this key
    pub fn toggle(self) -> Toggle<Self> {
        Toggle { input: self }
    }

    /// Set this key
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the key (`set_to(true)`)
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the key (`set_to(false)`)
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }
}

impl Input for MouseButton {}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
