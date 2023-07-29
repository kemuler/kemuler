//! Module of helper combinators

use core::fmt;
use std::{thread, time::Duration};

use crate::{input_event::Invert, simulatable::Simulatable};

#[cfg(test)]
mod test;

/// Helper combinator trait.
pub trait Combine: Sized {
    /// Simulate `self` and then `next`
    fn then<S>(self, next: S) -> Sequence<(Self, S)> {
        Sequence((self, next))
    }

    /// Simulate `self` and then sleep for duration
    fn sleep(self, duration: Duration) -> Sequence<(Self, Sleep)> {
        self.then(Sleep(duration))
    }

    /// Simulate `self` and then sleep for duration in milliseconds
    fn sleep_ms(self, duration: u64) -> Sequence<(Self, Sleep)> {
        self.sleep(Duration::from_millis(duration))
    }

    /// Repeat simulation for amount of times
    fn repeat(self, times: usize) -> Repeat<Self> {
        Repeat {
            times,
            simulate: self,
        }
    }

    /// Iterate through an iterator and simulate each item
    /// Self must be an iterator and its item must be `Simulatable`.
    fn iter_seq(self) -> IterSequence<Self>
    where
        Self: IntoIterator,
    {
        IterSequence { iter: self }
    }

    /// Simulate through a tuple starting from `.0`.
    /// Self must be a tuple with 0 <= size <= 32.
    /// Nest them if you ever need more.
    fn seq(self) -> Sequence<Self> {
        Sequence(self)
    }

    /// Simulate self during an event.
    /// After self is simulated, the *during* event is inverted
    /// and simulated at the end.
    fn during<DS>(self, during: DS) -> During<DS, Self> {
        During {
            during,
            simulate: self,
        }
    }
}

impl<S> Combine for S {}

/// Thread sleep for amount of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sleep(pub Duration);

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
        write!(f, "[sleep {} ms]", self.0.as_millis())
    }
}

/// Simulate an input for amount of times
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeat<S> {
    pub simulate: S,
    pub times: usize,
}

impl<S, Smlt> Simulatable<Smlt> for Repeat<S>
where
    S: Simulatable<Smlt> + Clone,
{
    fn run_with(self, simulator: &mut Smlt) {
        for _ in 0..self.times {
            self.simulate.clone().run_with(simulator)
        }
    }
}

impl<S> fmt::Display for Repeat<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[for {} times do ({})]", self.times, self.simulate)
    }
}

/// Simulate through a tuple starting from `.0`.
/// Supported size: 0 <= size <= 32
/// Nest them if you ever need more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sequence<T>(T);

/// put statments in reverse order
macro_rules! reverse_order {
    () => {};
    (
        $stmt:stmt; $($repeat_stmt:stmt;)*
    ) => {
        reverse_order!($($repeat_stmt;)*);
        $stmt
    };
}

/// implement sequence for n amount of tuples
macro_rules! tuple_impl {
    ($($n:tt => $g:ident,)*) => {
        tuple_impl!{@impl $($n => $g,)*}
        tuple_impl!{@cut_one $($n => $g,)*}
    };
    (@cut_one) => {};
    (@cut_one $_n:tt => $_g:ident, $($n:tt => $g:ident,)* ) => {
        tuple_impl!{@impl $($n => $g,)*}
        tuple_impl!{@cut_one $($n => $g,)*}
    };
    (@impl $($n:tt => $g:ident,)*) => {
        impl<Smlt, $($g,)*> Simulatable<Smlt> for Sequence<($($g,)*)>
        where
            $(
                $g: Simulatable<Smlt>,
            )*
        {
            #[allow(unused)]
            fn run_with(self, simulator: &mut Smlt) {
                let inner= self.0;
                reverse_order!(
                    $(
                        tuple_impl!(@nth inner, $n).run_with(simulator);
                    )*
                );
            }
        }

        impl<$($g,)*> fmt::Display for Sequence<($($g,)*)>
        where
            $(
                $g: fmt::Display,
            )*
        {
            #[allow(unused)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let inner = &self.0;
                reverse_order!(
                    $(
                        write!(f, "{}", tuple_impl!(@nth inner, $n))?;
                    )*
                );
                Ok(())
            }
        }
    };
    (@nth $x:ident, $n:tt) => {
        ($x.$n)
    };
    () => {}
}

// what a fat one
tuple_impl! {
    31 => I31, 30 => I30, 29 => I29, 28 => I28,
    27 => I27, 26 => I26, 25 => I25, 24 => I24,
    23 => I23, 22 => I22, 21 => I21, 20 => I20,
    19 => I19, 18 => I18, 17 => I17, 16 => I16,
    15 => I15, 14 => I14, 13 => I13, 12 => I12,
    11 => I11, 10 => I10, 9 => I9, 8 => I8,
    7 => I7, 6 => I6, 5 => I5, 4 => I4,
    3 => I3, 2 => I2, 1 => I1, 0 => I0,
}

/// Automatically do a for loop on an iterator and simulate for you!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IterSequence<I> {
    iter: I,
}

impl<I, Smlt> Simulatable<Smlt> for IterSequence<I>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Simulatable<Smlt>,
{
    fn run_with(self, simulator: &mut Smlt) {
        for s in self.iter {
            s.run_with(simulator);
        }
    }
}

impl<S> fmt::Display for IterSequence<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[iterate ({})]", self.iter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct During<DS, S> {
    during: DS,
    simulate: S,
}

impl<WS, S, Smlt> Simulatable<Smlt> for During<WS, S>
where
    S: Simulatable<Smlt>,
    WS: Invert + Simulatable<Smlt> + Clone,
    <WS as Invert>::Output: Simulatable<Smlt>,
{
    fn run_with(self, simulator: &mut Smlt) {
        self.during.clone().run_with(simulator);
        self.simulate.run_with(simulator);
        self.during.invert().run_with(simulator);
    }
}

impl<DS, S> fmt::Display for During<DS, S>
where
    DS: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[during ({}), do ({})]", self.during, self.simulate)
    }
}
