use super::*;
use core::fmt;

/// Accurate thread sleep for amount of time using [`spin_sleep`](https://crates.io/crates/spin_sleep).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpinSleep(pub Duration);

impl SpinSleep {
    pub fn new(duration: Duration) -> SpinSleep {
        SpinSleep(duration)
    }
}

impl<Smlt> Simulatable<Smlt> for SpinSleep {
    fn run_with(self, _: &mut Smlt) {
        ::spin_sleep::sleep(self.0)
    }
}

impl fmt::Display for SpinSleep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[spin sleep {} ms]", self.0.as_millis())
    }
}

impl From<Sleep> for SpinSleep {
    fn from(value: Sleep) -> Self {
        SpinSleep::new(value.0)
    }
}

impl From<Duration> for SpinSleep {
    fn from(value: Duration) -> Self {
        SpinSleep::new(value)
    }
}
