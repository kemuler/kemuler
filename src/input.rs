//! This module contains the `Input` trait and other things that (should) implements it.

use std::fmt;

use crate::simulate;

pub trait Input: Sized {
    /// Select a simulator for this input.
    fn simulate_with<S: simulate::Simulator<Self>>(self, simulator: &S) -> simulate::Simulate;
    /// Select a simulator for this input and execute it directly.
    fn execute_with<S: simulate::Simulator<Self>>(self, simulator: &mut S) {
        simulator.simulate_input(self)
    }
}

impl<T> Input for T {
    fn simulate_with<S: simulate::Simulator<Self>>(self, simulator: &S) -> simulate::Simulate {
        simulator.build_simulate(self)
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
