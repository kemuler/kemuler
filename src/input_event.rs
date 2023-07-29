//! Module containing base input events.

use crate::{simulatable::Simulatable, simulator::Simulate};
use std::fmt;

/// Trait for an event that is the inverse version of self.
/// For example, key down is the inverse version of key up and vice versa.
///
/// The inverse must completely annihilated each other
/// as if `a + b = 0` where `b` is the inverse of `a`.
///
/// This is only implemented on `SetTo<I, bool>` for now since that's
/// the only thing that is make sense to be implemented.
pub trait Invert {
    type Output;
    fn invert(self) -> Self::Output;
}

/// An event of some input state is going to be setted to some value.
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

impl<I> Invert for SetTo<I, bool> {
    type Output = SetTo<I, bool>;

    fn invert(self) -> Self::Output {
        let SetTo { input, to } = self;
        SetTo { input, to: !to }
    }
}

impl<I, V> fmt::Display for SetTo<I, V>
where
    I: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[set {} to {}]", self.input, self.to)
    }
}

/// An event of some input state is going to be changed by some value.
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
        write!(f, "[change {} by {}]", self.input, self.by)
    }
}
