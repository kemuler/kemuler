//! Collection of built-in simulator/backend implemented by this crate

#[cfg(feature = "enigo")]
pub use self::enigo::Enigo;

#[cfg(feature = "enigo")]
pub mod enigo {
    //! Simulate input using `Enigo`.

    use crate::quick_impl_simulatable;
    use crate::{
        input::{ChangeBy, SetTo},
        inputs::common::*,
        simulate::{Simulate, Simulator},
    };

    macro_rules! enigo_build_simulate_impl {
        ($ty:ty) => {
            fn build_simulate(&self, input: $ty) -> Simulate {
                quick_impl_simulatable!(
                    C ["Enigo"] { input: $ty } = (&self) {
                        Enigo.simulate_input(self.input)
                    }
                )
            }
        };
    }

    /// Simulate input using `Enigo`.
    #[derive(Debug, Clone, Copy)]
    pub struct Enigo;

    impl Simulator<SetTo<Key, bool>> for Enigo {
        enigo_build_simulate_impl! {SetTo<Key, bool>}

        fn simulate_input(&mut self, input: SetTo<Key, bool>) {
            println!("{input}");
        }
    }

    impl Simulator<ChangeBy<Key, bool>> for Enigo {
        enigo_build_simulate_impl! {ChangeBy<Key, bool>}

        fn simulate_input(&mut self, input: ChangeBy<Key, bool>) {
            println!("{input}");
        }
    }
}
