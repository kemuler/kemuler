//! Module for implementing a simulator

/// Trait to implement for a simulator.
/// It may support many type of input (`E`).
pub trait Simulate<S> {
    /// Simulate this simulatable
    fn simulate(&mut self, simulatable: S);
}
