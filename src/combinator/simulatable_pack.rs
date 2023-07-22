use std::fmt;

use crate::{
    prelude::Input,
    simulate::{Simulatable, Simulator},
};

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
    fn simulate(&mut self) {
        self.0.call()
    }
}

pub trait InputPacker<I> {
    fn pack_input(&self, input: I) -> SimulatablePack;
}

pub trait PackWith: Sized {
    fn pack_with<P: InputPacker<Self>>(self, packer: &P) -> SimulatablePack;
}

impl<I> PackWith for I
where
    I: Input,
{
    fn pack_with<P: InputPacker<Self>>(self, packer: &P) -> SimulatablePack {
        packer.pack_input(self)
    }
}
