//! Module of simulaties

/// A `Simulator` is a thing that knows how to simulate an input.
/// It may support many type of input (`I`).
pub trait Simulator<I> {
    fn run(&mut self, input: I);
}
