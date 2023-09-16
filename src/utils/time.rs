use std::time::{Duration, TryFromFloatSecsError};

pub fn secs(t: u64) -> Duration {
    Duration::from_secs(t)
}

pub fn secs_f64(t: f64) -> Duration {
    Duration::from_secs_f64(t)
}

pub fn secs_f32(t: f32) -> Duration {
    Duration::from_secs_f32(t)
}

pub fn try_secs_f64(t: f64) -> Result<Duration, TryFromFloatSecsError> {
    Duration::try_from_secs_f64(t)
}

pub fn try_secs_f32(t: f32) -> Result<Duration, TryFromFloatSecsError> {
    Duration::try_from_secs_f32(t)
}

pub fn millis(t: u64) -> Duration {
    Duration::from_millis(t)
}

pub fn micros(t: u64) -> Duration {
    Duration::from_micros(t)
}

pub fn nanos(t: u64) -> Duration {
    Duration::from_nanos(t)
}
