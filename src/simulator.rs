//! Module for implementing a simulator

/// Trait to implement for a simulator.
/// It may support many type of input (`E`).
pub trait Simulate<S> {
    /// Simulate this simulatable
    fn simulate(&mut self, simulatable: S);
}

impl<S, Smltr0, Smltr1> Simulate<S> for (&mut Smltr0, &mut Smltr1)
where
    S: Clone,
    Smltr0: Simulate<S>,
    Smltr1: Simulate<S>,
{
    fn simulate(&mut self, simulatable: S) {
        self.0.simulate(simulatable.clone());
        self.1.simulate(simulatable);
    }
}
