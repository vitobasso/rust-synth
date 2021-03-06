use std::time::{Duration, Instant};
use std::ops::Mul;
use super::Millis;
use crate::util;

pub struct Pulse {
    pub period: Duration,
    latest: Instant,
}

impl Pulse {

    pub fn new_with_millis(period: Millis) -> Self {
        Pulse::new(Duration::from_millis(period))
    }

    pub fn new(period: Duration) -> Self {
        Pulse{ period, latest: Instant::now() }
    }

    pub fn read(&mut self) -> Option<PulseReading> {
        let elapsed = self.latest.elapsed();
        let periods_passed = util::duration::div_duration(elapsed, self.period).floor() as u32;
        if periods_passed > 0 {
            let latest = self.latest + self.period.mul(periods_passed);
            let missed = (periods_passed - 1).max(0);
            let reading = PulseReading{ latest, missed };
            self.latest = latest;
            Some(reading)
        } else { None }
    }

    pub fn with_period(&self, period: Duration) -> Self {
        Pulse { period, latest: self.latest }
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct PulseReading {
    pub latest: Instant,
    pub missed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use std::ops::Sub;

    #[test]
    fn read_too_early() {
        let mut pulse = Pulse { period: Duration::from_millis(1000), latest: Instant::now() };
        assert_eq!(pulse.read(), None);
    }

    #[test]
    fn read_in_time() {
        let past_instant = Instant::now().sub(Duration::from_millis(1500));
        let mut pulse = Pulse { period: Duration::from_millis(1000), latest: past_instant };
        match pulse.read() {
            Some(PulseReading { latest, missed }) => {
                let elapsed = util::duration::as_float_secs(latest.elapsed());
                assert!(elapsed > 0. && elapsed < 2000.);
                assert_eq!(missed, 0);
            },
            None => assert!(false)
        }
    }

    #[test]
    fn read_too_late() {
        let past_instant = Instant::now().sub(Duration::from_millis(2500));
        let mut pulse = Pulse { period: Duration::from_millis(1000), latest: past_instant };
        match pulse.read() {
            Some(PulseReading { latest, missed }) => {
                let elapsed = util::duration::as_float_secs(latest.elapsed());
                assert!(elapsed > 0. && elapsed < 2000.);
                assert_eq!(missed, 1);
            },

            None => assert!(false)
        }
    }
}