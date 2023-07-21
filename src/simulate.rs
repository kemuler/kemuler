//! Module that contains the `Simulator` trait and other things that implements it.
#![macro_use]

use std::fmt;

/// Helps implement `Simulator`s
#[macro_export]
macro_rules! generate_build_simulatable_pack_impl {
    ([$display:expr] => $new_macro_ident:ident) => {
        macro_rules! $new_macro_ident {
            (
                $inner_ident:ident {
                    $input_ident:ident: $input_ty:ty
                }
                ($self:ident) $body:block
            ) => {
                fn pack_input(&self, input: $input_ty) -> SimulatablePack {
                    #[derive(::std::fmt::Debug, Clone, Copy)]
                    struct $inner_ident {
                        input: $input_ty,
                    }
                    impl $crate::simulate::SimulatablePackInner for $inner_ident {
                        fn call(&mut $self) $body
                    }
                    impl ::std::fmt::Display for $inner_ident {
                        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            ::std::write!(f, "{} {}", $display, self.input)
                        }
                    }
                    $crate::simulate::SimulatablePack::new($inner_ident { input })
                }
            };
        }
    };
}

/// A `Simulatable` is a thing that can be run to simulate an input.
pub trait Simulatable {
    /// Start simulating input.
    fn simulate(&mut self);
}

/// A `Simulator` is a simulator/backend that knows how to simulate an input.
/// It may support many kind of input (`I`).
///
/// If you want to add state to your `Simulator`, consider using [`std::sync::Arc`]
/// due to how the API is formed in the current state.
pub trait Simulator<I> {
    fn simulate_input(&mut self, input: I);
    /// Select this simulator for the input and comebine it to [`SimulatablePack`].
    fn pack_input(&self, input: I) -> SimulatablePack;
}

/// This trait is for implementing in `Simulator` that `SimulatablePack` required.
pub trait SimulatablePackInner: dyn_clone::DynClone + fmt::Display + fmt::Debug {
    fn call(&mut self);
}

/// An input with selected simulator and is ready to be simulated.
/// To simulate the input use `Executable::execute`.
pub struct SimulatablePack(Box<dyn SimulatablePackInner>);

impl SimulatablePack {
    /// Create new `SimulatablePack`.
    pub fn new<S>(s: S) -> SimulatablePack
    where
        S: SimulatablePackInner + 'static,
    {
        SimulatablePack(Box::new(s))
    }
}

impl Clone for SimulatablePack {
    fn clone(&self) -> Self {
        SimulatablePack(dyn_clone::clone_box(&*self.0))
    }
}

impl fmt::Debug for SimulatablePack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for SimulatablePack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Simulatable for SimulatablePack {
    /// Execute `Simulate` to simulate the input.
    fn simulate(&mut self) {
        self.0.call()
    }
}
