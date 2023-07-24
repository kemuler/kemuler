//! Module of helper combinators

use core::fmt;
use std::{thread, time::Duration};

use crate::event::Simulatable;

/// Helper combinator trait.
pub trait Combine: Sized {
    /// `self` and then `next`
    fn then<I>(self, next: I) -> AndThen<Self, I> {
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
pub struct AndThen<A, B>(A, B);

impl<A, B, S> Simulatable<S> for AndThen<A, B>
where
    A: Simulatable<S>,
    B: Simulatable<S>,
{
    fn run_with(self, simulator: &mut S) {
        self.0.run_with(simulator);
        self.1.run_with(simulator);
    }
}

impl<A, B> fmt::Display for AndThen<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.0, self.1)
    }
}

/// Thread sleep for amount of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sleep(Duration);

impl<S> Simulatable<S> for Sleep {
    fn run_with(self, _: &mut S) {
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
pub struct OnlyIf<T>(T, bool);

impl<T, S> Simulatable<S> for OnlyIf<T>
where
    T: Simulatable<S>,
{
    fn run_with(self, simulator: &mut S) {
        if self.1 {
            self.0.run_with(simulator)
        }
    }
}

impl<T> fmt::Display for OnlyIf<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 {
            write!(f, "run {};", self.0)
        } else {
            write!(f, "don't run {};", self.0)
        }
    }
}
