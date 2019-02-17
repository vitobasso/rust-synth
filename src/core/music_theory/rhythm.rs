use super::diatonic_scale::{RelativePitch, OctaveShift, ScaleDegree};
use crate::util::reckless_float::RecklessFloat;
use std::collections::BTreeMap;

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

#[derive(Clone)]
pub struct Phrase {
    notes: BTreeMap<RecklessFloat, Note>,
}

impl Phrase {
    pub fn new(notes: &[Note]) -> Self {
        let total_duration: u32 = notes.iter().map(|n| n.duration as u32).sum();
        let map: BTreeMap<RecklessFloat, Note> = notes.iter()
            .scan(0., |progress, note| {
                let index = RecklessFloat(*progress);
                *progress += note.duration as u32 as f64 / total_duration as f64;
                Some((index, *note))
            }).collect();
        Phrase { notes: map }
    }

    /// `from` and `to` are expected to be between 0 and 1. TODO restrictive newtype
    /// Values outside this range will result in an empty `Vec`.
    pub fn range(&self, from: f64, to: f64) -> Vec<Note> {
        if from <= to {
            self.notes
                .range(RecklessFloat(from)..RecklessFloat(to))
                .map(|(_, v)| *v).collect()
        } else {
            let first_half = self.notes
                .range(RecklessFloat(from)..)
                .map(|(_, v)| *v);
            let second_half = self.notes
                .range(..RecklessFloat(to))
                .map(|(_, v)| *v);
            first_half.chain(second_half).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::NoteDuration::*;
    use super::super::diatonic_scale::{ScaleDegree::*, OctaveShift::*};

    const A: Note = Note { duration: Half,    pitch: (Same, I1) };
    const B: Note = Note { duration: Quarter, pitch: (Same, I2) };
    const C: Note = Note { duration: Quarter, pitch: (Same, I3) };

    #[test]
    fn whole() {
        let result = Phrase::new(&[A, B, C]).range(0., 1.);
        assert_eq!(result, vec!(A, B, C))
    }

    #[test]
    fn beginning() {
        let result = Phrase::new(&[A, B, C]).range(0., 0.5);
        assert_eq!(result, vec!(A))
    }

    #[test]
    fn end() {
        let result = Phrase::new(&[A, B, C]).range(0.4, 1.);
        assert_eq!(result, vec!(B, C))
    }

    #[test]
    fn middle() {
        let result = Phrase::new(&[A, B, C]).range(0.5, 0.6);
        assert_eq!(result, vec!(B))
    }

    #[test]
    fn wrapping() {
        let result = Phrase::new(&[A, B, C]).range(0.9, 0.1);
        assert_eq!(result, vec!(A))
    }

    #[test]
    fn invalid_range() {
        let result = Phrase::new(&[A, B, C]).range(1., 2.);
        assert_eq!(result, vec!())
    }

    #[test]
    fn invalid_and_valid_ranges() {
        let result = Phrase::new(&[A, B, C]).range(-1., 2.);
        assert_eq!(result, vec!(A, B, C))
    }

}