//! Module of simulaties

/// A `Simulatable` is a thing that can be run to simulate an input without a simulator.
pub trait ReadySimulate {
    /// Start simulating input.
    fn run(&mut self);
}

/// A `Simulator` is a thing that knows how to simulate an input.
/// It may support many type of input (`I`).
pub trait Simulator<I> {
    fn run(&mut self, input: I);
}
