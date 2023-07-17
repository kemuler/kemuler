//! This module contains the `Input` trait and other things that (should) implements it.

use std::fmt;

use crate::simulate;

/// Modules of inputs that is generally found/a standard.
pub mod common {
    use super::*;

    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub enum Key {
        A,
        B,
        C,
        D,
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
