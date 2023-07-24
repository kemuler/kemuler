//! This module contains the `Input` trait and other things that (should) implements it.

use std::fmt;

use crate::simulate::Simulator;

pub trait InputEvent: Sized {
    /// Simulate input directly.
    fn run_with<S: Simulator<Self>>(self, simulator: &mut S) {
        simulator.run(self)
    }
}

/// Let [`simulate::Simulator`] knows that you want to toggle a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
