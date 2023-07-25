//! This module contains things that can be simulated

use std::fmt;

use crate::simulator::Simulate;

/// Simulatable is a thing that can be simulated by a simulator.
/// Alternatively called an event.
pub trait Simulatable<Smlt>: Sized {
    /// Simulate this input.
    fn run_with(self, simulator: &mut Smlt);
}

/// Let a simulator knows that you want to set the value of an input to a value.
/// The simulator must support this to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetTo<I, V> {
    pub input: I,
    pub to: V,
}

impl<I, V, Smlt> Simulatable<Smlt> for SetTo<I, V>
where
    Smlt: Simulate<Self>,
{
    fn run_with(self, simulator: &mut Smlt) {
        simulator.simulate(self)
    }
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

/// Let simulator knows that you want to change the value of an input by a value.
/// The simulator must support this to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChangeBy<I, V> {
    pub input: I,
    pub by: V,
}

impl<I, V, Smlt> Simulatable<Smlt> for ChangeBy<I, V>
where
    Smlt: Simulate<Self>,
{
    fn run_with(self, simulator: &mut Smlt) {
        simulator.simulate(self)
    }
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
