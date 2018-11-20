use std::time::{Duration, SystemTime};
use super::rhythm::*;
use super::diatonic_scale::*;

const PULSE_RATE: Duration = Duration::from_millis(100);

pub struct Pulse {
    latest: SystemTime,
}
impl Pulse {
    pub fn new() -> Pulse {
        Pulse{ latest: SystemTime::now() }
    }
    pub fn read(&mut self) -> Option<SystemTime> {
        match self.latest.elapsed() {
            Ok(elapsed) => {
                let should_trigger = elapsed >= PULSE_RATE;
                if should_trigger {
                    let current_pulse = self.latest + PULSE_RATE;
                    self.latest = current_pulse;
                    Some(current_pulse)
                } else { None }
            },
            _ => panic!("Failed to read clock")
        }
    }
}