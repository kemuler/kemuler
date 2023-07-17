//! Module that contains the `Simulator` trait and other things that implements it.

use std::fmt;

use crate::execute::Executable;

/// Help implement `Simulate` with less boilerplate
#[macro_export]
macro_rules! impl_simulatable {
    (
        for $thing:ident;$($tails:tt)+
    ) => {
        impl_simulatable!(@impls($thing) $($tails)+)
    };
    (
        @impls($thing:ident) fn Simulatable::call(&$arg0:ident) $body:block $($tails:tt)*
    ) => {
        impl $crate::simulate::Simulatable for $thing {
            fn call(&mut $arg0) $body
        }
        impl_simulatable!(@impls($thing) $($tails)*)
    };
    (
        @impls($thing:ident) fn Display::fmt(&$arg0:ident, $arg1:ident) $body:block $($tails:tt)*
    ) => {
        impl ::std::fmt::Display for $thing {
            fn fmt(&$arg0, $arg1: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result $body
        }

        impl_simulatable!(@impls($thing) $($tails)*)
    };
    (
        @impls($thing:ident) fn Debug::fmt(&arg0, $arg1:ident) $body:block $($tails:tt)*
    ) => {
        impl ::std::fmt::Debug for $thing {
            fn fmt(&$arg0, $arg1: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result $body
        }

        impl_simulatable!(@impls($thing) $($tails)*)
    };
    (@impls($thing:ident)) => {};
    () => {};
}

/// Help implement `Simulate` with muchhhh less boilerplate
#[macro_export]
macro_rules! quick_impl_simulatable {
    (
        $ident:ident [$display:expr] {
            $input_ident:ident: $input_ty:ty
        } = (&$self_arg:ident) $body:block
    ) => {{
        #[derive(Debug, Clone, Copy)]
        struct $ident {
            $input_ident: $input_ty,
        }
        $crate::impl_simulatable! {
            for $ident;
            fn Simulatable::call(&$self_arg) $body
            fn Display::fmt(&self, f) {
                ::std::write!(f, "{} {}", $display, self.$input_ident)
            }
        }
        $crate::simulate::Simulate::new($ident { $input_ident })
    }};
}

/// A `Simulator` is a simulator/backend that knows how to simulate an input.
/// It may support many kind of input (`I`).
///
/// If you want to add state to your `Simulator`, consider using `Arc`
/// due to how the API is formed in the current state.
pub trait Simulator<I> {
    /// Select this simulator for the input.
    fn simulate_input(&mut self, input: I);
    fn build_simulate(&self, input: I) -> Simulate;
}

/// This trait is for implementing in `Simulator` that `Simulate` required.
pub trait Simulatable: dyn_clone::DynClone + fmt::Display + fmt::Debug {
    fn call(&mut self);
}

/// An input with selected simulator.
/// An input will not simulate until execute.
/// To execute use `Executable::execute`.
pub struct Simulate(Box<dyn Simulatable>);

impl Simulate {
    /// Create new `Simulate`.
    pub fn new<S>(s: S) -> Simulate
    where
        S: Simulatable + 'static,
    {
        Simulate(Box::new(s))
    }
}

impl Clone for Simulate {
    fn clone(&self) -> Self {
        Simulate(dyn_clone::clone_box(&*self.0))
    }
}

impl fmt::Debug for Simulate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for Simulate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Executable for Simulate {
    /// Execute `Simulate` to simulate the input.
    fn execute(&mut self) {
        self.0.call()
    }
}
