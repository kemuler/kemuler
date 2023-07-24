//! Module of helper combinators

use core::fmt;
use std::{thread, time::Duration};

use crate::event::Event;

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
}

impl<T> Combine for T {}

/// Simulate 2 input consecutively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndThen<A, B>(A, B);

impl<A, B, S> Event<S> for AndThen<A, B>
where
    A: Event<S>,
    B: Event<S>,
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

impl<S> Event<S> for Sleep {
    fn run_with(self, _: &mut S) {
        thread::sleep(self.0);
    }
}

impl fmt::Display for Sleep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sleep {} ms;", self.0.as_millis())
    }
}
