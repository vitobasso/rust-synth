
use super::diatonic_scale::ScalePitch;

#[derive(Clone, Copy, Debug)]
pub enum Duration {
    Whole=16, Half=8, Quarter=4, Eight=2, Sixteenth=1
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    Note(ScalePitch),
    Rest,
    Keep,
}

#[derive(Clone, Copy, Debug)]
pub struct Note {
    duration: Duration,
    pitch: Option<ScalePitch>,
}

impl Note {
    pub fn note(duration: Duration, pitch: ScalePitch) -> Note {
        Note { duration, pitch: Some(pitch) }
    }
    pub fn rest(duration: Duration) -> Note {
        Note { duration, pitch: None }
    }
    fn events(self) -> Vec<Event> {
        let head = match self.pitch {
            Some(pitch) => Event::Note(pitch),
            None => Event::Rest,
        };
        let len = self.duration as u32;
        let tail: Vec<Event> = (1..len).map(|_| Event::Keep).collect();

        vec![head].into_iter().chain(tail).collect()
    }
}

#[derive(Clone, Debug)]
pub struct Sequence {
    measures: u32,
    pub events: Vec<Event>,
}

impl Sequence {

    pub fn new(measures: u32, notes: Vec<Note>) -> Result<Sequence, Invalid> {
        let events = notes.iter().flat_map(|note| note.events()).collect();
        let seq = Sequence { measures, events };
        seq.validate(notes).map(|_| seq)
    }

    fn validate(&self, notes: Vec<Note>) -> Result<(), Invalid> {
        let total_duration = notes.iter()
            .fold(0, |acc, note| acc + note.duration as u32);
        let expected_total = self.measures * 16;
        if total_duration == expected_total {
            Ok(())
        } else {
            Err(Invalid::IncompleteSeq { expected: expected_total, actual: total_duration })
        }
    }
}

#[derive(Debug)]
pub enum Invalid {
    IncompleteSeq{ expected: u32, actual: u32 }
}



#[cfg(test)]
mod tests {
    use super::{Sequence, Event::{self, *}, Note, Duration::{self, *}, Invalid};
    use super::super::diatonic_scale::{ScalePitch, Octave::*, ScaleDegree::*};

    const W: Note = Note{duration: Whole,     pitch: None};
    const H: Note = Note{duration: Half,      pitch: None};
    const Q: Note = Note{duration: Quarter,   pitch: None};
    const E: Note = Note{duration: Eight,     pitch: None};
    const S: Note = Note{duration: Sixteenth, pitch: None};

    #[test]
    fn sequence_validation() {
        let cases: &[(u32, Vec<Note>, bool)] = &[
            (1, vec![W],          true),
            (1, vec![H, H],       true),
            (1, vec![H, Q, Q],    true),
            (1, vec![Q, Q, Q, Q], true),
            (1, vec![Q, Q, Q],    false),
            (1, vec![Q, W],       false),
            (2, vec![W, W],       true),
            (2, vec![H, W, H],    true),
            (2, vec![H, W, Q],    false),
            (2, vec![W, W, S],    false),
        ];
        for (measures, notes, should_be_valid) in cases.iter() {
            let actual_result = Sequence::new(*measures, notes.clone()).is_ok();
            assert_eq!(actual_result, *should_be_valid,
                       "Input was: {:?}, {:?}, {:?}", measures, notes, *should_be_valid);
        }
    }

    #[test]
    fn sequence_rest_events() {
        let cases: &[(&[Note], &[Event])] = &[
            (&[W],              &[Rest, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Keep]),
            (&[H, H],           &[Rest, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Rest, Keep, Keep, Keep, Keep, Keep, Keep, Keep]),
            (&[H, Q, Q],        &[Rest, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Rest, Keep, Keep, Keep, Rest, Keep, Keep, Keep]),
            (&[H, Q, E, S, S],  &[Rest, Keep, Keep, Keep, Keep, Keep, Keep, Keep, Rest, Keep, Keep, Keep, Rest, Keep, Rest, Rest]),
        ];
        for (notes, expected_events) in cases.iter() {
            let sequence = Sequence::new(1, notes.to_vec())
                .expect("Expected a valid Sequence");
            let actual_result = sequence.events;
            assert_eq!(actual_result, expected_events.to_vec(),
                       "Input was: {:?}, {:?}", notes, *expected_events);
        }
    }

    #[test]
    fn sequence_a_phrase() {
        let phrase = &[
            Note::note(Quarter, (Same, I1)),
            Note::rest(Quarter),
            Note::note(Quarter, (Same, I2)),
            Note::note(Half,    (Same, I3)),
            Note::rest(Quarter),
            Note::note(Quarter, (Same, I2)),
            Note::note(Quarter, (Same, I1)),
        ];
        let sequence = Sequence::new(2, phrase.to_vec())
            .expect("Expected a valid Sequence");
        let expected_events = &[
            Event::Note((Same, I1)), Keep, Keep, Keep,
            Rest, Keep, Keep, Keep,
            Event::Note((Same, I2)), Keep, Keep, Keep,
            Event::Note((Same, I3)), Keep, Keep, Keep, Keep, Keep, Keep, Keep,
            Rest, Keep, Keep, Keep,
            Event::Note((Same, I2)), Keep, Keep, Keep,
            Event::Note((Same, I1)), Keep, Keep, Keep,
        ];
        let actual_result = sequence.events;
        assert_eq!(actual_result, expected_events.to_vec(),
                   "Input was: {:?}, {:?}", phrase, *expected_events);
    }

}