//! Module of simulaties

/// Trait to implement for a `Simulator`.
/// It may support many type of input (`E`).
pub trait Simulate<E> {
    fn run(&mut self, event: E);
}
