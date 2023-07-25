//! Module of helper combinators

use core::fmt;
use std::{thread, time::Duration};

use crate::simulatable::Simulatable;

/// Helper combinator trait.
pub trait Combine: Sized {
    /// `self` and then `next`
    fn then<S>(self, next: S) -> AndThen<Self, S> {
        AndThen(self, next)
    }

    /// `self` and then sleep for amount of time
    /// This is a convenience shorthand, see code for more details.
    fn sleep(self, duration: Duration) -> AndThen<Self, Sleep> {
        self.then(Sleep(duration))
    }

    /// `self` and then sleep for amount of time in milliseconds
    /// This is a convenience shorthand, see code for more details.
    fn sleep_ms(self, duration: u64) -> AndThen<Self, Sleep> {
        self.sleep(Duration::from_millis(duration))
    }

    fn only_if(self, condition: bool) -> OnlyIf<Self> {
        OnlyIf(self, condition)
    }
}

impl<T> Combine for T {}

/// Simulate 2 input consecutively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndThen<SA, SB>(SA, SB);

impl<SA, SB, Smlt> Simulatable<Smlt> for AndThen<SA, SB>
where
    SA: Simulatable<Smlt>,
    SB: Simulatable<Smlt>,
{
    fn run_with(self, simulator: &mut Smlt) {
        self.0.run_with(simulator);
        self.1.run_with(simulator);
    }
}

impl<SA, SB> fmt::Display for AndThen<SA, SB>
where
    SA: fmt::Display,
    SB: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.0, self.1)
    }
}

/// Thread sleep for amount of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sleep(Duration);

impl<Smlt> Simulatable<Smlt> for Sleep {
    fn run_with(self, _: &mut Smlt) {
        thread::sleep(self.0);
    }
}

impl fmt::Display for Sleep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sleep {} ms;", self.0.as_millis())
    }
}

/// Simulate if the closure evaluated to true
/// Useful with conditional compilation. Not sure about other stuff.
pub struct OnlyIf<S>(S, bool);

impl<S, Smlt> Simulatable<Smlt> for OnlyIf<S>
where
    S: Simulatable<Smlt>,
{
    fn run_with(self, simulator: &mut Smlt) {
        if self.1 {
            self.0.run_with(simulator)
        }
    }
}

impl<S> fmt::Display for OnlyIf<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 {
            write!(f, "run {};", self.0)
        } else {
            write!(f, "don't run {};", self.0)
        }
    }
}
