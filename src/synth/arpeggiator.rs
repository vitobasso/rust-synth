use std::time::{Duration, SystemTime};
use super::rhythm::{*, Duration::*};
use super::diatonic_scale::{*, Octave::*, ScaleDegree::*};
use super::pitch::{Pitch, PitchClass};
use super::super::controller::Command;

pub struct Arpeggiator {
    sequence: Sequence,
    index: usize,
    key: Key,
    holding: Option<Pitch>,
    playing: Option<Pitch>,
}
impl Arpeggiator {
    pub fn new(key: Key, sequence: Sequence) -> Arpeggiator {
        Arpeggiator {
            sequence,
            index: 0,
            key,
            holding: None,
            playing: None,
        }
    }
    pub fn start(&mut self, pitch: Pitch) {
        self.holding = Some(pitch);
    }

    pub fn stop(&mut self) {
        self.holding = None;
        self.playing = None;
    }

    pub fn is_holding(&self, pitch: Pitch) -> bool {
        self.holding.map(|p| p == pitch).unwrap_or(false)
    }

    pub fn next(&mut self) -> Option<Command> {
        self.next_event()
            .and_then(|e| self.update_and_command(e))
    }

    fn next_event(&mut self) -> Option<Event> {
        let events = &self.sequence.events;
        self.index = (self.index + 1) % events.len();
        events.iter().nth(self.index).cloned()
    }

    fn update_and_command(&mut self, event: Event) -> Option<Command> {
        match (event, self.holding, self.playing) {
            (Event::Note(relative_pitch), Some(holding), _) => {
                self.key.pitch_at(holding, relative_pitch).map(|pitch| {
                    self.playing = Some(pitch);
                    Command::ArpNoteOn(pitch)
                })
            },
            (Event::Rest, _, Some(playing)) => {
                self.playing = None;
                Some(Command::ArpNoteOff(playing))
            }
            _ => None,
        }
    }
}

pub struct Builder { pub key: PitchClass, pub sequence: Sequence }
impl Builder {
    pub fn new(key: PitchClass, sequence: Sequence) -> Builder {
        Builder { key, sequence }
    }
    pub fn build(&self) -> Arpeggiator {
        Arpeggiator::new(self.key, self.sequence.clone())
    }

    pub fn preset_1() -> Builder {
        Builder::new(
            PitchClass::C,
            Sequence::new(1, vec![
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
            ]).expect("Invalid sequence")
        )
    }

    pub fn preset_2() -> Builder {
        Builder::new(
            PitchClass::C,
            Sequence::new(1, vec![
                Note::note(Eight, (Down2, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Same, I1)),
                Note::note(Eight, (Down1, I1)),
                Note::note(Eight, (Down2, I1)),
                Note::note(Eight, (Down1, I1)),
            ]).expect("Invalid sequence")
        )
    }

    pub fn preset_3() -> Builder {
        Builder::new(
            PitchClass::C,
            Sequence::new(1, vec![
                Note::note(Sixteenth, (Down1, I1)),
                Note::note(Sixteenth, (Down1, I3)),
                Note::note(Sixteenth, (Down1, I5)),
                Note::note(Sixteenth, (Same, I1)),
                Note::note(Sixteenth, (Same, I3)),
                Note::note(Sixteenth, (Same, I5)),
                Note::note(Sixteenth, (Up1, I1)),
                Note::note(Sixteenth, (Up1, I3)),
                Note::note(Sixteenth, (Up1, I5)),
                Note::note(Sixteenth, (Up1, I3)),
                Note::note(Sixteenth, (Up1, I1)),
                Note::note(Sixteenth, (Same, I5)),
                Note::note(Sixteenth, (Same, I3)),
                Note::note(Sixteenth, (Same, I1)),
                Note::note(Sixteenth, (Down1, I5)),
                Note::note(Sixteenth, (Down1, I3)),
            ]).expect("Invalid sequence")
        )
    }
}