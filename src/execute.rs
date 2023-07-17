//! This module contains the `Executable` trait and other useful combinators.

use std::time::Duration;

/// An `Executable` is a thing that can be run to simulate an input.
// (Suppose to anyways. Nothing prevents some random guy from doing other random stuff)
pub trait Executable {
    fn execute(&mut self);
}

/// Helper combinator trait
pub trait Combinator: Sized {
    /// Queue `self` and then `next`
    fn and_then<EB>(self, next: EB) -> AndThen<Self, EB>;

    /// Queue `self` and then `Sleep`
    /// Convenience shorthand for `and_then(Sleep(...))`
    fn and_sleep(self, sleep: Duration) -> AndThen<Self, Sleep>;

    /// Only execute if on Windows
    fn win_os(self) -> WinOs<Self>;

    /// Only execute if on Unix
    fn unix_os(self) -> UnixOs<Self>;
}

// impl for T without trait bounds is possible
// but the combinator really only works with things that implements `Executable`
impl<T> Combinator for T
where
    T: Executable,
{
    fn and_then<EB>(self, next: EB) -> AndThen<Self, EB> {
        AndThen(self, next)
    }

    /// Queue `self` and then `Sleep`
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

impl Executable for Sleep {
    fn execute(&mut self) {
        std::thread::sleep(self.0)
    }
}

impl<E: Executable> Executable for Option<E> {
    fn execute(&mut self) {
        if let Some(e) = self {
            e.execute()
        }
    }
}

/// Queue `EA` and then `EB`
pub struct AndThen<EA, EB>(EA, EB);

impl<EA, EB> Executable for AndThen<EA, EB>
where
    EA: Executable,
    EB: Executable,
{
    fn execute(&mut self) {
        self.0.execute();
        self.1.execute();
    }
}

/// Only execute if on Windows
pub struct WinOs<E>(E);

impl<E> Executable for WinOs<E>
where
    E: Executable,
{
    fn execute(&mut self) {
        if cfg!(window) {
            self.0.execute()
        }
    }
}

/// Only execute if on Unix
pub struct UnixOs<E>(E);

impl<E> Executable for UnixOs<E>
where
    E: Executable,
{
    fn execute(&mut self) {
        if cfg!(unix) {
            self.0.execute()
        }
    }
}
