//! Simulate input using `Enigo`.

use crate::{
    input::{ChangeBy, SetTo},
    inputs::common,
    simulate::Simulator,
};
use enigo::KeyboardControllable;

/// Simulate input using `Enigo`.
#[derive(Debug, Clone, Copy)]
pub struct Enigo(enigo::Enigo);

impl Simulator<SetTo<enigo::Key, bool>> for Enigo {
    fn simulate_input(&mut self, input: SetTo<enigo::Key, bool>) {
        let SetTo { input, to } = input;
    }
}
