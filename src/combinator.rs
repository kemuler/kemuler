//! This module contains combinators.

use crate::simulate::Simulatable;
use std::time::Duration;

pub mod simulatable_pack;

/// Helper combinator trait
pub trait Combine: Sized {
    /// Queue `self` and then `next`
    fn and_then<EB>(self, next: EB) -> AndThen<Self, EB> {
        AndThen(self, next)
    }

    /// Queue `self` and then `Sleep`
    /// Convenience shorthand for `and_then(Sleep(...))`
    fn and_sleep(self, duration: Duration) -> AndThen<Self, Sleep> {
        AndThen(self, Sleep(duration))
    }

    /// Queue `self` and then `Sleep` in milliseconds
    /// Convenience shorthand for `and_then(Sleep(Duration::from_millis(...)))`
    fn and_sleep_ms(self, duration: u64) -> AndThen<Self, Sleep> {
        self.and_sleep(Duration::from_millis(duration))
    }

    fn and_if(self, bool: bool) -> AndIf<Self> {
        AndIf(self, bool)
    }
}

impl<T: Simulatable> Combine for T {}

/// Sleep for amount of duration
pub struct Sleep(Duration);

impl Simulatable for Sleep {
    fn simulate(&mut self) {
        std::thread::sleep(self.0)
    }
}

/// Simulate `.0` then `.1`
pub struct AndThen<SA, SB>(SA, SB);

impl<SA, SB> Simulatable for AndThen<SA, SB>
where
    SA: Simulatable,
    SB: Simulatable,
{
    fn simulate(&mut self) {
        self.0.simulate();
        self.1.simulate();
    }
}

/// Simulate only if bool is `true`
/// Useful with conditional compilation.
pub struct AndIf<S>(S, bool);

impl<S> Simulatable for AndIf<S>
where
    S: Simulatable,
{
    fn simulate(&mut self) {
        if self.1 {
            self.0.simulate()
        }
    }
}
