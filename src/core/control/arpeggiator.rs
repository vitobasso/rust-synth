use core::control::{Millis, pulse::Pulse, instrument_player::Command};
use core::music_theory::{pitch::Pitch, rhythm::{Sequence, Event}, diatonic_scale::*};
use std::mem;

pub struct Arpeggiator {
    sequence: Sequence,
    index: usize,
    pulse: Pulse,
    key: Key,
    holding: Option<Pitch>,
    playing: Option<Pitch>,
    pending: Option<Command>,
}

const NOTE_ID: u32 = 10;

impl Arpeggiator {

    pub fn new(pulse_period: Millis, key: Key, sequence: Sequence) -> Arpeggiator {
        Arpeggiator {
            sequence, key,
            index: 0,
            pulse: Pulse::with_period_millis(pulse_period),
            holding: None,
            playing: None,
            pending: None,
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, _) => self.start(pitch),
            Command::NoteOff(pitch, _) => self.stop(pitch),
            other => panic!("Can't interpret command: {:?}", other)
        }
    }

    fn start(&mut self, pitch: Pitch) {
        self.holding = Some(pitch);
    }

    fn stop(&mut self, pitch: Pitch) {
        if self.is_holding(pitch) {
            self.pending = self.playing.map(|p| Command::NoteOff(p, NOTE_ID));
            self.holding = None;
            self.playing = None;
        }
    }

    fn is_holding(&self, pitch: Pitch) -> bool {
        self.holding.map(|p| p == pitch).unwrap_or(false)
    }

    pub fn next(&mut self) -> Vec<Command> {
        match mem::replace(&mut self.pending, None) {
            Some(pending) => vec![pending],
            None => self.pulse.read()
                        .and_then(|_| self.next_event())
                        .map(|e| self.update_and_command(e))
                        .unwrap_or(vec![]),
        }
    }

    fn next_event(&mut self) -> Option<Event> {
        let events = &self.sequence.events;
        self.index = (self.index + 1) % events.len();
        events.iter().nth(self.index).cloned()
    }

    fn update_and_command(&mut self, event: Event) -> Vec<Command> {
        match (event, self.holding, self.playing) {
            (Event::Note(relative_pitch), Some(holding), None) =>
                self.update_note_on(relative_pitch, holding).into_iter().collect(),
            (Event::Note(relative_pitch), Some(holding), Some(playing)) =>
                vec![ Some(note_off(playing)),
                      self.update_note_on(relative_pitch, holding),
                ].into_iter().flatten().collect(),
            (Event::Rest, None, Some(playing)) => {
                self.playing = None;
                vec![note_off(playing)]
            }
            _ => vec![],
        }
    }

    fn update_note_on(&mut self, relative_pitch: RelativePitch, holding: Pitch) -> Option<Command> {
        let next_pitch = self.key.pitch_at(holding, relative_pitch);
        self.playing = next_pitch;
        next_pitch.map(note_on)
    }

    pub fn set_pulse(&mut self, period: Millis) {
        self.pulse = Pulse::with_period_millis(period)
    }

}

fn note_on(pitch: Pitch) -> Command {
    Command::NoteOn(pitch, NOTE_ID)
}

fn note_off(pitch: Pitch) -> Command {
    Command::NoteOff(pitch, NOTE_ID)
}
