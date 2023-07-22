//! Simulate input using `Enigo`.

use crate::{
    input::{ChangeBy, SetTo},
    inputs::common::*,
    simulate::Simulator,
};

/// Simulate input using `Enigo`.
#[derive(Debug, Clone, Copy)]
pub struct Enigo;

impl Simulator<SetTo<Keyboard, bool>> for Enigo {
    fn simulate_input(&mut self, input: SetTo<Keyboard, bool>) {
        println!("{input}");
    }
}

impl Simulator<ChangeBy<Keyboard, bool>> for Enigo {
    fn simulate_input(&mut self, input: ChangeBy<Keyboard, bool>) {
        println!("{input}");
    }
}
