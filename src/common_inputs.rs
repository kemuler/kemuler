//! Generally found/a standard input
//!
//! This is currently pretty much incomplete.
//! Prefer `enigo` and their `Key` and `MouseButton` for most cases.

use crate::{combinator::*, input_event::*};
use std::fmt;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// alt key on Linux and Windows (option key on macOS)
    // Alt, LAlt, RAlt,
    Alt,
    Shift,
    Control,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    CapsLock,

    End, Home, PageUp, PageDown,

    Escape, Return, Space, Tab,

    Backspace, Delete,

    UpArrow, DownArrow, LeftArrow, RightArrow,
}

impl Key {
    /// Set this key
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = Key::Alt;
    /// # let to = true;
    /// SetTo { input: this, to: to }
    /// # ;
    /// ```
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the key.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = Key::Alt;
    /// SetTo { input: this, to: true }
    /// # ;
    /// ```
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the key
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = Key::Alt;
    /// SetTo { input: this, to: false }
    /// # ;
    /// ```
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the key.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = Key::Alt;
    /// (
    ///     SetTo { input: this, to: true },
    ///     SetTo { input: this, to: false }
    /// ).seq()
    /// # ;
    /// ```
    pub fn click(self) -> Sequence<(SetTo<Self, bool>, SetTo<Self, bool>)> {
        self.down().then(self.up())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MousePosition;

impl MousePosition {
    /// Move mouse to x, y
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MousePosition;
    /// # let x = 10i32;
    /// # let y = 10i32;
    /// SetTo { input: this, to: (x, y) }
    /// # ;
    /// ```
    pub fn move_to(self, x: i32, y: i32) -> SetTo<Self, (i32, i32)> {
        SetTo {
            input: self,
            to: (x, y),
        }
    }

    /// Move mouse by x, y (move mouse relatively)
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MousePosition;
    /// # let x = 10i32;
    /// # let y = 10i32;
    /// ChangeBy { input: this, by: (x, y) }
    /// # ;
    /// ```
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
    /// Scroll mouse wheel by x, y.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseScroll;
    /// # let x = 10i32;
    /// # let y = 10i32;
    /// ChangeBy { input: this, by: (x, y) }
    /// # ;
    /// ```
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
#[non_exhaustive]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    /// Set this button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// # let to = true;
    /// SetTo { input: this, to: to }
    /// # ;
    /// ```
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the button.
    /// This is a convenience shorthand for
    /// ```
    /// # let this = MouseButton::Left;
    /// SetTo { input: this, to: true }
    /// # ;
    /// ```
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// SetTo { input: this, to: false }
    /// # ;
    /// ```
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// (
    ///     SetTo { input: this, to: true },
    ///     SetTo { input: this, to: false }
    /// ).seq()
    /// # ;
    /// ```
    pub fn click(self) -> Sequence<(SetTo<Self, bool>, SetTo<Self, bool>)> {
        self.down().then(self.up())
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
