//! Simulate input using `Enigo`.

use crate::generate_build_simulatable_pack_impl;
use crate::{
    input::{ChangeBy, SetTo},
    inputs::common::*,
    simulate::Simulator,
};

generate_build_simulatable_pack_impl! {["Enigo"] => enigo_build_impl}

/// Simulate input using `Enigo`.
#[derive(Debug, Clone, Copy)]
pub struct Enigo;

impl Simulator<SetTo<Key, bool>> for Enigo {
    enigo_build_impl! {
        EnigoSetKeyto { input: SetTo<Key, bool> }
        (self) {
            Enigo.simulate_input(self.input)
        }
    }

    fn simulate_input(&mut self, input: SetTo<Key, bool>) {
        println!("{input}");
    }
}

impl Simulator<ChangeBy<Key, bool>> for Enigo {
    enigo_build_impl! {
        EnigoChangeKeyBy { input: ChangeBy<Key, bool> }
        (self) {
            Enigo.simulate_input(self.input)
        }
    }

    fn simulate_input(&mut self, input: ChangeBy<Key, bool>) {
        println!("{input}");
    }
}
