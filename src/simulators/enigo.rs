//! Simulate input using `Enigo`.

use crate::{
    input::{ChangeBy, SetTo},
    inputs::common::*,
    simulate::Simulator,
};

/// Simulate input using `Enigo`.
#[derive(Debug, Clone, Copy)]
pub struct Enigo;

impl Simulator<SetTo<Key, bool>> for Enigo {
    fn simulate_input(&mut self, input: SetTo<Key, bool>) {
        println!("{input}");
    }
}

impl Simulator<ChangeBy<Key, bool>> for Enigo {
    fn simulate_input(&mut self, input: ChangeBy<Key, bool>) {
        println!("{input}");
    }
}
