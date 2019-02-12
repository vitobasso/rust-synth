use std::time::{Duration, Instant};
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
                Some(duration_as_millis(end - begin)),
            _ => None,
        }
    }
}

//TODO replace with Duration.as_millis after rust 1.33
fn duration_as_millis(duration: Duration) -> Millis {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    secs * 1_000 + u64::from(millis)
}