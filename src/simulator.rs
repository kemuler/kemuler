//! Module for implementing a simulator

/// Trait to implement for a simulator.
/// It may support many type of input (`E`).
pub trait Simulate<S> {
    /// Simulate this simulatable
    fn simulate(&mut self, simulatable: S);
}

impl<S, Smlt0, Smlt1> Simulate<S> for (&mut Smlt0, &mut Smlt1)
where
    S: Clone,
    Smlt0: Simulate<S>,
    Smlt1: Simulate<S>,
{
    fn simulate(&mut self, simulatable: S) {
        self.0.simulate(simulatable.clone());
        self.1.simulate(simulatable);
    }
}
