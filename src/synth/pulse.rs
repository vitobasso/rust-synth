use std::time::{Duration, SystemTime};
use super::rhythm::*;
use super::diatonic_scale::*;

pub struct Pulse {
    period: Duration,
    latest: SystemTime,
}
impl Pulse {
    pub fn with_period_millis(period_millis: u64) -> Pulse {
        Pulse::with_period(Duration::from_millis(period_millis))
    }
    pub fn with_period(period: Duration) -> Pulse {
        Pulse{ period, latest: SystemTime::now() }
    }
    pub fn read(&mut self) -> Option<SystemTime> {
        match self.latest.elapsed() {
            Ok(elapsed) => {
                let should_trigger = elapsed >= self.period;
                if should_trigger {
                    let current_pulse = self.latest + self.period;
                    self.latest = current_pulse;
                    Some(current_pulse)
                } else { None }
            },
            _ => panic!("Failed to read clock")
        }
    }
}