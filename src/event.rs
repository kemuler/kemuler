//! This module contains the `Simulatable` trait and other things that implements it.

use std::fmt;

use crate::simulate::Simulate;

/// Simulatable is a thing that can be simulated by a simulator.
/// Alternatively called an event.
pub trait Simulatable<S>: Sized {
    /// Simulate this input.
    fn run_with(self, simulator: &mut S);
}

/// Simulatable is a thing that can be simulated by a simulator.
/// Alternatively called an event.
/// This is falliable version for [`Simulatable`]
pub trait TrySimulatable<S>: Sized {
    type Error;
    /// Try simulate this input.
    fn try_run_with(self, simulator: &mut S) -> Result<(), (Self::Error, Self)>;
}

impl<T, S> TrySimulatable<S> for T
where
    S: Simulate<T>,
{
    type Error = void::Void;

    fn try_run_with(self, simulator: &mut S) -> Result<(), (Self::Error, Self)> {
        simulator.run(self);
        Ok(())
    }
}

/// Let [`Simulator`] knows that you want to set the value of an input to a value.
/// Simulator must support this to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetTo<I, V> {
    pub input: I,
    pub to: V,
}

impl<I, V, S> Simulatable<S> for SetTo<I, V>
where
    S: Simulate<Self>,
{
    fn run_with(self, simulator: &mut S) {
        simulator.run(self)
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

/// Let [`Simulator`] knows that you want to change the value of an input by a value.
/// Simulator must support this to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChangeBy<I, V> {
    pub input: I,
    pub by: V,
}

impl<I, V, S> Simulatable<S> for ChangeBy<I, V>
where
    S: Simulate<Self>,
{
    fn run_with(self, simulator: &mut S) {
        simulator.run(self)
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
