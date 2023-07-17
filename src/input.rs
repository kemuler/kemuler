//! This module contains the `Input` trait and other things that (should) implements it.

use std::fmt;

use crate::{prelude::Executable, simulate};

/// Modules of inputs that is generally found/a standard.
pub mod common {
    use super::*;

    #[rustfmt::skip]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
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

    impl fmt::Display for Key {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
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
    }
}

pub trait Input: Sized {
    /// Select a simulator for this input.
    fn simulate_with<S: simulate::Simulator<Self>>(self, simulator: &S) -> simulate::Simulate;
    /// Select a simulator for this input and execute it directly.
    fn execute_with<S: simulate::Simulator<Self>>(self, simulator: &S) {
        simulator.quick_execute(self)
    }
}

impl<T> Input for T {
    fn simulate_with<S: simulate::Simulator<Self>>(self, simulator: &S) -> simulate::Simulate {
        simulator.simulate_input(self)
    }
}

/// Let [`simulate::Simulator`] knows that you want to toggle a key.
#[derive(Debug, Clone, Copy)]
pub struct Toggle<I> {
    pub input: I,
}

impl<I> fmt::Display for Toggle<I>
where
    I: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "toggle {}", self.input)
    }
}

/// Let [`simulate::Simulator`] knows that you want to set the value of a key to a value.
#[derive(Debug, Clone, Copy)]
pub struct SetTo<I, V> {
    pub input: I,
    pub to: V,
}

impl<I, V> fmt::Display for SetTo<I, V>
where
    I: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set {} to {}", self.input, self.to)
    }
}

/// Let [`simulate::Simulator`] knows that you want to change the value of a key by a value.
#[derive(Debug, Clone, Copy)]
pub struct ChangeBy<I, V> {
    pub input: I,
    pub by: V,
}

impl<I, V> fmt::Display for ChangeBy<I, V>
where
    I: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "change {} by {}", self.input, self.by)
    }
}
