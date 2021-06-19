use std::time::Instant;
use super::Millis;

#[derive(Default)]
pub struct TapTempo {
    begin: Option<Instant>,
    end: Option<Instant>,
}

impl TapTempo {

    pub fn tap(&mut self) {
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
                Some((end - begin).as_millis() as u64),
            _ => None,
        }
    }
}
