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

    /// Repeat simulation for amount of times
    fn repeat(self, times: usize) -> Repeat<Self> {
        Repeat(self, times)
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

impl Sleep {
    pub fn from_ms(ms: u64) -> Sleep {
        Sleep(Duration::from_millis(ms))
    }
}

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

/// Simulate an input for amount of times
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeat<S>(S, usize);

impl<S, Smlt> Simulatable<Smlt> for Repeat<S>
where
    S: Simulatable<Smlt> + Clone,
{
    fn run_with(self, simulator: &mut Smlt) {
        for _ in 0..self.1 {
            self.0.clone().run_with(simulator)
        }
    }
}

impl<S> fmt::Display for Repeat<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} times;", self.0, self.1)
    }
}

macro_rules! tuple_impl {
    ($($n:tt => $g:ident,)*) => {
        tuple_impl!{@impl $($n => $g,)*}
        tuple_impl!{@cut_one $($n => $g,)*}
    };
    (@cut_one ) => {};
    (@cut_one $cn:tt => $cg:ident,) => {};
    (@cut_one $_n:tt => $_g:ident, $($n:tt => $g:ident,)* ) => {
        tuple_impl!{@impl $($n => $g,)*}
        tuple_impl!{@cut_one $($n => $g,)*}
    };
    (@impl $($n:tt => $g:ident,)*) => {
        impl<Smlt, $($g,)*> Simulatable<Smlt> for ($($g,)*)
        where
            $(
                $g: Simulatable<Smlt>,
            )*
        {
            fn run_with(self, simulator: &mut Smlt) {
                $(
                    tuple_impl!(@nth $n, self).run_with(simulator);
                )*
            }
        }
    };
    (@nth $n:tt, $x:ident) => {
        ($x.$n)
    };
    () => {}
}

// what a fat one
tuple_impl! {
    63 => I63, 62 => I62, 61 => I61, 60 => I60,
    59 => I59, 58 => I58, 57 => I57, 56 => I56,
    55 => I55, 54 => I54, 53 => I53, 52 => I52,
    51 => I51, 50 => I50, 49 => I49, 48 => I48,
    47 => I47, 46 => I46, 45 => I45, 44 => I44,
    43 => I43, 42 => I42, 41 => I41, 40 => I40,
    39 => I39, 38 => I38, 37 => I37, 36 => I36,
    35 => I35, 34 => I34, 33 => I33, 32 => I32,
    31 => I31, 30 => I30, 29 => I29, 28 => I28,
    27 => I27, 26 => I26, 25 => I25, 24 => I24,
    23 => I23, 22 => I22, 21 => I21, 20 => I20,
    19 => I19, 18 => I18, 17 => I17, 16 => I16,
    15 => I15, 14 => I14, 13 => I13, 12 => I12,
    11 => I11, 10 => I10, 9 => I9, 8 => I8,
    7 => I7, 6 => I6, 5 => I5, 4 => I4,
    3 => I3, 2 => I2, 1 => I1, 0 => I0,
}

impl<Smlt, S, const N: usize> Simulatable<Smlt> for [S; N]
where
    S: Simulatable<Smlt>,
{
    fn run_with(self, simulator: &mut Smlt) {
        for s in self.into_iter() {
            s.run_with(simulator);
        }
    }
}
