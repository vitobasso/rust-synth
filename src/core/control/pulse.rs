use std::time::{Duration, Instant};
use super::Millis;

pub struct Pulse {
    period: Duration,
    latest: Instant,
}

impl Pulse {

    pub fn with_period_millis(period: Millis) -> Pulse {
        Pulse::with_period(Duration::from_millis(period))
    }

    pub fn with_period(period: Duration) -> Pulse {
        Pulse{ period, latest: Instant::now() }
    }

    pub fn read(&mut self) -> Option<Instant> {
        let should_trigger = self.latest.elapsed() >= self.period;
        if should_trigger {
            let current_pulse = self.latest + self.period;
            self.latest = current_pulse;
            Some(current_pulse)
        } else { None }
    }

}