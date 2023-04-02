use std::time::Duration;

pub trait IntoDuration {
    fn into_duration(self) -> Duration;
}

impl IntoDuration for Duration {
    fn into_duration(self) -> Duration {
        self
    }
}

impl IntoDuration for f64 {
    fn into_duration(self) -> Duration {
        Duration::from_secs_f64(self)
    }
}

impl IntoDuration for f32 {
    fn into_duration(self) -> Duration {
        Duration::from_secs_f32(self)
    }
}

impl IntoDuration for u64 {
    fn into_duration(self) -> Duration {
        Duration::from_secs(self)
    }
}

pub trait Wait {
    fn wait<D: IntoDuration>(&self, duration: D) -> &Self {
        wait(duration);
        self
    }

    fn and_then<F: FnOnce(&Self)>(&self, then: F) -> &Self {
        then(self);
        self
    }
}

pub fn wait<D: IntoDuration>(duration: D) {
    std::thread::sleep(duration.into_duration());
}

impl<T> Wait for T {}
