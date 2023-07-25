//! Module of simulaties

// (is it bit empty? yeah. it's reserved for other wip branches)

/// Trait to implement for a simulator.
/// It may support many type of input (`E`).
pub trait Simulate<S> {
    /// Simulate this simulatable
    fn simulate(&mut self, simulatable: S);
}
