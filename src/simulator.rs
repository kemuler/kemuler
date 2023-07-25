//! Module of simulaties

/// Trait to implement for a simulator.
/// It may support many type of input (`E`).
pub trait Simulate<S> {
    fn simulate(&mut self, simulaltable: S);
}

// pub trait Simulator {
//     // fn run(&mut self, )
// }
