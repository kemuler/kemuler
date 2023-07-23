//! Module that contains the `Simulator` trait and other things that implements it.

/// A `Simulatable` is a thing that can be run to simulate an input.
pub trait Simulatable {
    /// Start simulating input.
    fn simulate(&mut self);
}

/// A `Simulator` is a thing that knows how to simulate an input.
/// It may support many type of input (`I`).
pub trait Simulator<I> {
    fn run(&mut self, input: I);
}
