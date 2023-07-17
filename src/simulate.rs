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
            #[derive(Debug, Clone, Copy)]
            struct SomeStruct {
                input: SetTo<Key, bool>,
            }
            impl_simulatable! {
                for SomeStruct;
                fn Simulatable::call(&self) {
                    println!("{self}");
                }
                fn Display::fmt(&self, f) {
                    write!(f, "{}", self.input)
                }
            }
            Simulate::new(SomeStruct { input })
        }
    }

    impl Simulator<ChangeBy<Key, bool>> for Enigo {
        fn simulate_input(&self, input: ChangeBy<Key, bool>) -> Simulate {
            #[derive(Debug, Clone, Copy)]
            struct SomeStruct {
                input: ChangeBy<Key, bool>,
            }
            impl_simulatable! {
                for SomeStruct;
                fn Simulatable::call(&self) {
                    println!("{self}");
                }
                fn Display::fmt(&self, f) {
                    write!(f, "{}", self.input)
                }
            }
            Simulate::new(SomeStruct { input })
        }
    }
}
