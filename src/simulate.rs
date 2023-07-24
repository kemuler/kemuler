//! Module of simulaties

/// Trait to implement for a `Simulator`.
/// It may support many type of input (`E`).
pub trait Simulate<E> {
    fn run(&mut self, event: E);
}

/// Trait to implement for a `Simulator`.
/// It may support many type of input (`E`).
/// This is the falliable version of [`Simulate`]
pub trait TrySimulate<E> {
    type Error;
    fn try_run(&mut self, event: E) -> Result<(), (Self::Error, E)>;
}
