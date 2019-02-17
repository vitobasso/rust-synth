use std::time::Instant;
use crate::util;
use super::Millis;

#[derive(Default)]
pub struct DurationRecorder {
    begin: Option<Instant>,
    end: Option<Instant>,
}

impl DurationRecorder {

    pub fn record(&mut self) {
        let now = Instant::now();
        match (self.begin, self.end) {
            (Some(begin), Some(end)) if begin < end && end < now => {
                self.begin = Some(end);
                self.end = Some(now);
            },
            (Some(begin), _) if begin < now =>
                self.end = Some(now),
            _ =>
                self.begin = Some(now),
        }
    }

    pub fn read(&self) -> Option<Millis> {
        match (self.begin, self.end) {
            (Some(begin), Some(end)) if begin < end =>
                Some(util::duration::as_millis(end - begin)),
            _ => None,
        }
    }
}
