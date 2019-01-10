use core::control::{Millis, pulse::Pulse, instrument_player::{Command, Id, id}};
use core::music_theory::{pitch::Pitch, rhythm::{Sequence, Event}, diatonic_scale::*};
use std::mem;

pub struct Arpeggiator {
    sequence: Sequence,
    index: usize,
    pulse: Pulse,
    key: Key,
    holding_pitch: Option<Pitch>,
    playing_pitch: Option<Pitch>,
    pending_command: Option<Command>,
}

impl Arpeggiator {

    pub fn new(pulse_period: Millis, key: Key, sequence: Sequence) -> Arpeggiator {
        Arpeggiator {
            sequence, key,
            index: 0,
            pulse: Pulse::with_period_millis(pulse_period),
            holding_pitch: None,
            playing_pitch: None,
            pending_command: None,
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, _) => self.start(pitch),
            Command::NoteOff(id) => self.stop(id),
            other => panic!("Can't interpret command: {:?}", other)
        }
    }

    fn start(&mut self, pitch: Pitch) {
        self.holding_pitch = Some(pitch);
    }

    fn stop(&mut self, id: Id) {
        if self.is_holding(id) {
            self.pending_command = self.playing_pitch.map(note_off);
            self.holding_pitch = None;
            self.playing_pitch = None;
        }
    }

    fn is_holding(&self, id: Id) -> bool {
        self.holding_pitch.map(|p| p == id.pitch).unwrap_or(false)
    }

    pub fn next(&mut self) -> Vec<Command> {
        match mem::replace(&mut self.pending_command, None) {
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
        match (event, self.holding_pitch, self.playing_pitch) {
            (Event::Note(relative_pitch), Some(holding), None) =>
                self.update_note_on(relative_pitch, holding).into_iter().collect(),
            (Event::Note(relative_pitch), Some(holding), Some(playing)) =>
                vec![ Some(note_off(playing)),
                      self.update_note_on(relative_pitch, holding),
                ].into_iter().flatten().collect(),
            (Event::Rest, None, Some(playing)) => {
                self.playing_pitch = None;
                vec![note_off(playing)]
            }
            _ => vec![],
        }
    }

    fn update_note_on(&mut self, relative_pitch: RelativePitch, holding: Pitch) -> Option<Command> {
        let next_pitch = self.key.pitch_at(holding, relative_pitch);
        self.playing_pitch = next_pitch;
        next_pitch.map(note_on)
    }

    pub fn set_pulse(&mut self, period: Millis) {
        self.pulse = Pulse::with_period_millis(period)
    }

}

fn note_on(pitch: Pitch) -> Command {
    Command::NoteOn(pitch, id(pitch))
}

fn note_off(pitch: Pitch) -> Command {
    Command::NoteOff(id(pitch))
}
