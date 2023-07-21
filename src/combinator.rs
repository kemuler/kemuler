//! This module contains the `Executable` trait and other useful combinators.

use crate::simulate::Simulatable;
use std::time::Duration;

/// Helper combinator trait
pub trait Combinator: Sized {
    /// Queue `self` and then `next`
    fn and_then<EB>(self, next: EB) -> AndThen<Self, EB>;

    /// Queue `self` and then `Sleep`
    /// Convenience shorthand for `and_then(Sleep(...))`
    fn and_sleep(self, sleep: Duration) -> AndThen<Self, Sleep>;

    /// Only simulate this if on Windows
    fn win_os(self) -> WinOs<Self>;

    /// Only simulate this if on Unix
    fn unix_os(self) -> UnixOs<Self>;
}

// impl for T without `Simulatable` trait bound is possible
// but the combinator really only works with things that implements `Executable`
impl<T> Combinator for T
where
    T: Simulatable,
{
    fn and_then<EB>(self, next: EB) -> AndThen<Self, EB> {
        AndThen(self, next)
    }

    fn and_sleep(self, sleep: Duration) -> AndThen<Self, Sleep> {
        AndThen(self, Sleep(sleep))
    }

    fn win_os(self) -> WinOs<Self> {
        WinOs(self)
    }

    fn unix_os(self) -> UnixOs<Self> {
        UnixOs(self)
    }
}

/// Sleep for amount of duration
pub struct Sleep(Duration);

impl Simulatable for Sleep {
    fn simulate(&mut self) {
        std::thread::sleep(self.0)
    }
}

impl<E: Simulatable> Simulatable for Option<E> {
    fn simulate(&mut self) {
        if let Some(e) = self {
            e.simulate()
        }
    }
}

/// Queue `EA` and then `EB`
pub struct AndThen<EA, EB>(EA, EB);

impl<EA, EB> Simulatable for AndThen<EA, EB>
where
    EA: Simulatable,
    EB: Simulatable,
{
    fn simulate(&mut self) {
        self.0.simulate();
        self.1.simulate();
    }
}

/// Only simulate this input if on Windows
pub struct WinOs<E>(E);

impl<E> Simulatable for WinOs<E>
where
    E: Simulatable,
{
    fn simulate(&mut self) {
        if cfg!(window) {
            self.0.simulate()
        }
    }
}

/// Only simulate this input if on Unix
pub struct UnixOs<E>(E);

impl<E> Simulatable for UnixOs<E>
where
    E: Simulatable,
{
    fn simulate(&mut self) {
        if cfg!(unix) {
            self.0.simulate()
        }
    }
}
