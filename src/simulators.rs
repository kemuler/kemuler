//! Collection of built-in simulator/backend implemented by this crate

pub mod enigo {
    //! Simulate input using `Enigo`.
    use crate::{
        input::{ChangeBy, SetTo},
        inputs::common::*,
        simulate::{Simulate, Simulator},
    };

    macro_rules! enigo_build_simulate_impl {
        ($ty:ty, $input_ident:ident) => {
            fn build_simulate(&self, $input_ident: $ty) -> Simulate {
                quick_impl_simulatable!(
                    C ["Enigo"] { $input_ident: $ty } = (&self) {
                        Enigo.simulate_input(self.$input_ident)
                    }
                )
            }
        };
    }

    /// Simulate input using `Enigo`.
    #[derive(Debug, Clone, Copy)]
    pub struct Enigo;

    impl Simulator<SetTo<Key, bool>> for Enigo {
        enigo_build_simulate_impl! {SetTo<Key, bool>, input}

        fn simulate_input(&mut self, input: SetTo<Key, bool>) {
            println!("{input}");
        }
    }

    impl Simulator<ChangeBy<Key, bool>> for Enigo {
        enigo_build_simulate_impl! {ChangeBy<Key, bool>, input}

        fn simulate_input(&mut self, input: ChangeBy<Key, bool>) {
            println!("{input}");
        }
    }
}
