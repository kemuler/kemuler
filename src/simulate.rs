//! Module that contains the `Simulator` trait and other things that implements it.

use std::fmt;

use crate::execute::Executable;
use crate::input;

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
            fn call(&$arg0) $body
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
        @impls($thing:ident) fn Display::fmt(&$arg0:ident, $arg1:ident) lazy $($tails:tt)*
    ) => {
        impl ::std::fmt::Display for $thing {
            fn fmt(&$arg0, $arg1: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!($arg1, "<implementor is lazy, go complain in git issue>")
            }
        }

        impl_simulatable!(@impls($thing) $($tails)*)
    };
    (
        @impls($thing:ident) fn Display::fmt(&$arg0:ident, $arg1:ident) inner($use_inner:ident) $($tails:tt)*
    ) => {
        impl ::std::fmt::Display for $thing {
            fn fmt(&$arg0, $arg1: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!($arg1, "{}", $arg0.$use_inner)
            }
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
        $ident:ident{
            $input_ident:ident: $input_ty:ty
        } = (&$self_arg:ident) $body:block
    ) => {
        #[derive(Debug, Clone, Copy)]
        struct $ident {
            $input_ident: $input_ty,
        }
        impl_simulatable! {
            for $ident;
            fn Simulatable::call(&$self_arg) $body
            fn Display::fmt(&self, f) inner(input)
        }
        $crate::simulate::Simulate::new($ident { $input_ident })
    };
}

/// A `Simulator` is a simulator/backend that knows how to simulate an input.
/// It may support many kind of input (`I`).
pub trait Simulator<I> {
    /// Select this simulator for the input.
    fn simulate_input(&self, input: I) -> Simulate;
}

/// This trait is for implementing in `Simulator` that `Simulate` required.
pub trait Simulatable: dyn_clone::DynClone + fmt::Display + fmt::Debug {
    fn call(&self);
}

/// An input with selected simulator.
/// An input will not simulate until execute.
/// To execute use `Executable::execute`.
pub struct Simulate(Box<dyn Simulatable>);

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

impl Simulate {
    /// Create new `Simulate`.
    pub fn new<S>(s: S) -> Simulate
    where
        S: Simulatable + 'static,
    {
        Simulate(Box::new(s))
    }
}

impl Executable for Simulate {
    /// Execute `Simulate` to simulate the input.
    fn execute(&mut self) {
        self.0.call()
    }
}

/// Simulate input using `Enigo`.
pub mod enigo {
    use crate::input::ChangeBy;

    use super::{
        input::{common::Key, SetTo},
        Simulate, Simulator,
    };

    /// Simulate input using `Enigo`.
    #[derive(Debug, Clone, Copy)]
    pub struct Enigo;

    impl Simulator<SetTo<Key, bool>> for Enigo {
        fn simulate_input(&self, input: SetTo<Key, bool>) -> Simulate {
            quick_impl_simulatable! {
                SomeStruct { input: SetTo<Key, bool> } = (&self) {
                    println!("{self}");
                }
            }
        }
    }

    impl Simulator<ChangeBy<Key, bool>> for Enigo {
        fn simulate_input(&self, input: ChangeBy<Key, bool>) -> Simulate {
            quick_impl_simulatable! {
                SomeStruct { input: ChangeBy<Key, bool> } = (&self) {
                    println!("{self}");
                }
            }
        }
    }
}
