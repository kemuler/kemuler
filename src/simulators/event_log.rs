use core::fmt;
use std::any::Any;

use dyn_clone::DynClone;

use crate::simulate::Simulate;

pub trait EventLog: fmt::Display + Any + DynClone {
    fn as_any(&self) -> &dyn Any;
}

impl<T: fmt::Display + Any + DynClone> EventLog for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EventLogger<S> {
    simulator: S,
    events: Vec<Box<dyn EventLog>>,
}

impl<S> EventLogger<S> {
    pub fn events(&self) -> &[Box<dyn EventLog>] {
        &self.events
    }

    pub fn new(simulator: S) -> EventLogger<S> {
        EventLogger {
            simulator,
            events: vec![],
        }
    }
}

impl<S, E> Simulate<E> for EventLogger<S>
where
    E: EventLog,
    S: Simulate<E>,
{
    fn run(&mut self, event: E) {
        self.events.push(dyn_clone::clone_box(&event));
        self.simulator.run(event);
    }
}

impl<S> fmt::Display for EventLogger<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for event in &self.events {
            write!(f, "{}", event)?
        }
        Ok(())
    }
}
