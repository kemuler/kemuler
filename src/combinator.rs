use crate::{input::InputEvent, simulate::Simulator};

/// Combinator
trait Combine: Sized {
    fn and_then<I>(self, next: I) -> AndThen<Self, I> {
        AndThen(self, next)
    }
}

/// Simulate 2 input consecutively
pub struct AndThen<A, B>(A, B);

impl<A, B, S> InputEvent<S> for AndThen<A, B>
where
    S: Simulator<A> + Simulator<B>,
{
    fn run_with(self, simulator: &mut S) {
        simulator.run(self.0);
        simulator.run(self.1);
    }
}
