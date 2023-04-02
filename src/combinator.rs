use crate::emulatable::{EmulateAbsoluteValue, EmulateRelativeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct And<A, B>(pub A, pub B);

pub trait Combine {
    fn and<Rhs>(self, rhs: Rhs) -> And<Self, Rhs>
    where
        Self: Sized,
    {
        And(self, rhs)
    }
}

impl<A, B> Combine for And<A, B> {}

impl<V, A, B> EmulateAbsoluteValue for And<A, B>
where
    A: EmulateAbsoluteValue<Value = V>,
    B: EmulateAbsoluteValue<Value = V>,
    V: Clone,
{
    type Value = V;

    fn change_to(&self, to: Self::Value) -> &Self {
        self.0.change_to(to.clone());
        self.1.change_to(to);
        self
    }
}

impl<V, A, B> EmulateRelativeValue for And<A, B>
where
    A: EmulateRelativeValue<Value = V>,
    B: EmulateRelativeValue<Value = V>,
    V: Clone,
{
    type Value = V;

    fn change_by(&self, by: Self::Value) -> &Self {
        self.0.change_by(by.clone());
        self.1.change_by(by);
        self
    }
}
