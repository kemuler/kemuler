use crate::timing::IntoDuration;

pub trait EmulateBinaryState: EmulateAbsoluteValue<Value = bool> {
    fn activate(&self) -> &Self {
        self.change_to(true);
        self
    }

    fn deactivate(&self) -> &Self {
        self.change_to(false);
        self
    }

    fn pulse(&self) -> &Self {
        self.activate();
        self.deactivate();
        self
    }

    fn pulse_for<D: IntoDuration>(&self, duration: D) -> &Self {
        self.activate();
        std::thread::sleep(duration.into_duration());
        self.deactivate();
        self
    }
}

impl<T: EmulateAbsoluteValue<Value = bool>> EmulateBinaryState for T {}

pub trait EmulateAbsoluteValue {
    type Value;
    fn change_to(&self, to: Self::Value) -> &Self;
}

pub trait EmulateRelativeValue {
    type Value;
    fn change_by(&self, by: Self::Value) -> &Self;
}
