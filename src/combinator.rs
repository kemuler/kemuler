//! Module of helper combinators

use core::fmt;
use std::{thread, time::Duration};

use crate::{input_event::Invert, simulatable::Simulatable};

#[cfg(test)]
mod test;

#[cfg(feature = "spin_sleep")]
mod spin_sleep;

#[cfg(feature = "spin_sleep")]
pub use self::spin_sleep::SpinSleep;

/// Helper combinator trait.
pub trait Combine: Sized {
    /// Simulate `self` and then `next`
    fn then<S>(self, next: S) -> Sequence<(Self, S)> {
        Sequence((self, next))
    }

    /// Simulate `self` and then sleep for duration
    fn sleep(self, duration: Duration) -> Sequence<(Self, Sleep)> {
        self.then(Sleep::new(duration))
    }

    /// Simulate `self` and then spin sleep for duration
    #[cfg(feature = "spin_sleep")]
    fn spin_sleep(self, duration: Duration) -> Sequence<(Self, SpinSleep)> {
        self.then(SpinSleep::new(duration))
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
    fn during<DS>(self, during: DS) -> During<DS, Self>
    where
        DS: Invert + Clone,
    {
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
    pub fn new(duration: Duration) -> Sleep {
        Sleep(duration)
    }
}

impl<Smltr> Simulatable<Smltr> for Sleep {
    fn run_with(self, _: &mut Smltr) {
        thread::sleep(self.0);
    }
}

impl fmt::Display for Sleep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[sleep {} ms]", self.0.as_millis())
    }
}

impl From<Duration> for Sleep {
    fn from(value: Duration) -> Self {
        Sleep::new(value)
    }
}

/// Simulate an input for amount of times
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeat<S> {
    pub simulate: S,
    pub times: usize,
}

impl<S, Smltr> Simulatable<Smltr> for Repeat<S>
where
    S: Simulatable<Smltr> + Clone,
{
    fn run_with(self, simulator: &mut Smltr) {
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
pub struct Sequence<T>(pub T);

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
        impl<Smltr, $($g,)*> Simulatable<Smltr> for Sequence<($($g,)*)>
        where
            $(
                $g: Simulatable<Smltr>,
            )*
        {
            #[allow(unused)]
            fn run_with(self, simulator: &mut Smltr) {
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

impl<I, Smltr> Simulatable<Smltr> for IterSequence<I>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Simulatable<Smltr>,
{
    fn run_with(self, simulator: &mut Smltr) {
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

impl<DS, S, Smltr> Simulatable<Smltr> for During<DS, S>
where
    S: Simulatable<Smltr>,
    DS: Invert + Simulatable<Smltr> + Clone,
    <DS as Invert>::Output: Simulatable<Smltr>,
{
    fn run_with(self, simulator: &mut Smltr) {
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

/// Call a simulator with a closure
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Call<F>(F);

impl<Smltr, F> Simulatable<Smltr> for Call<F>
where
    F: FnMut(&mut Smltr),
{
    fn run_with(mut self, simulator: &mut Smltr) {
        self.0(simulator)
    }
}

impl<F> fmt::Display for Call<F>
where
    F: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[call {}]", self.0)
    }
}
