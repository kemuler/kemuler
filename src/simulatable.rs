//! Module for implementing a thing can be simulated

/// Simulatable is a thing that can be simulated by a simulator.
/// In this crate, it is implemented on combinators and input event.
pub trait Simulatable<Smlt>: Sized {
    /// Simulate this input.
    fn run_with(self, simulator: &mut Smlt);
}
