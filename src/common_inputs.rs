//! Generally found/a standard input
//!
//! This is currently pretty much incomplete.
//! Prefer `enigo` and their `Key` and `MouseButton` for most cases.

use crate::{combinator::*, input_event::*};
use std::fmt;

/// The whole thing is a convenience shorthand
pub trait ButtonLike: Sized {
    /// Set this button state
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::input_event::*;
    /// # let this = 0i32;
    /// # let to = 0i32;
    /// SetTo { input: this, to: to }
    /// # ;
    /// ```
    fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::input_event::*;
    /// # let this = 0i32;
    /// SetTo { input: this, to: true }
    /// # ;
    /// ```
    fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the key
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::input_event::*;
    /// # let this = 0i32;
    /// SetTo { input: this, to: false }
    /// # ;
    /// ```
    fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the button consecutively.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = 0i32;
    /// (
    ///     SetTo { input: this, to: true },
    ///     SetTo { input: this, to: false }
    /// ).seq()
    /// # ;
    /// ```
    fn click(self) -> Sequence<(SetTo<Self, bool>, SetTo<Self, bool>)>
    where
        Self: Clone,
    {
        self.clone().down().then(self.up())
    }
}

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    Shift,
    Control,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    CapsLock,

    End, Home, PageUp, PageDown,

    Escape, Enter, Space, Tab,

    Backspace, Delete,

    UpArrow, DownArrow, LeftArrow, RightArrow,
}

impl ButtonLike for Key {}

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
    /// # let output =
    /// SetTo { input: this, to: (x, y) }
    /// # ;
    /// # assert_eq!(this.move_to(x, y), output);
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
    /// # let output =
    /// ChangeBy { input: this, by: (x, y) }
    /// # ;
    /// # assert_eq!(this.move_by(x, y), output);
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
    /// # let output =
    /// ChangeBy { input: this, by: (x, y) }
    /// # ;
    /// # assert_eq!(this.scroll_by(x, y), output);
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

impl ButtonLike for MouseButton {}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Char(pub char);

impl ButtonLike for Char {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrSequence<'a>(pub &'a str);

/// This is a convenience shorthand for
/// ```
/// # use kemuler::{prelude::*, input_event::*};
/// # let this = StrSequence("abcdefg");
/// # let output =
/// Execute { input: this }
/// # ;
/// # assert_eq!(this.execute(), output);
/// ```
impl<'a> StrSequence<'a> {
    pub fn execute(self) -> Execute<Self> {
        Execute { input: self }
    }
}
