use super::diatonic_scale::{RelativePitch, OctaveShift, ScaleDegree};
use crate::util::range_map::RangeMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoteDuration {
    Whole=16, Half=8, Quarter=4, Eight=2, Sixteenth=1
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Note {
    pub duration: NoteDuration,
    pub pitch: RelativePitch,
}

impl Note {
    pub fn new(duration: NoteDuration, octave: OctaveShift, degree: ScaleDegree) -> Self {
        Note { duration, pitch: (octave, degree) }
    }
}

/// Range is valid between 0 and 1. TODO restrictive newtype
/// Values outside this range will result in an empty `Vec`.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Phrase {
    map: RangeMap<Note>,
}

impl Phrase {
    pub fn new(notes: &[Note]) -> Self {
        let total_duration: u32 = notes.iter().map(|n| n.duration as u32).sum();
        let pairs: Vec<(f64, Note)> = notes.iter()
            .scan(0., |progress, note| {
                let index = *progress;
                *progress += note.duration as u32 as f64 / total_duration as f64;
                Some((index, *note))
            }).collect();
        Phrase { map: RangeMap::new(pairs) }
    }

    pub fn range(&self, from: f64, to: f64) -> Vec<Note> {
        self.map.range(from, to).into_iter().cloned().collect()
    }
}