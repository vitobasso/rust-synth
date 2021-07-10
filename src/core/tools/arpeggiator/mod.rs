use std::mem;

use crate::core::control::{synth::{Command, Id, id}};
use crate::core::sheet_music::sheet_music::MeasurePosition;
use crate::core::music_theory::{diatonic_scale::*, pitch::Pitch, rhythm::Note};

pub mod builder;
pub mod phrase;

pub struct Arpeggiator {
    phrase: Phrase,
    pub key: Key,
    holding_pitch: Option<Pitch>,
    playing_pitch: Option<Pitch>,
    pending_command: Option<Command>,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Specs {
    pub key: Key,
    pub phrase: builder::Specs,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub phrase: phrase::View,
    pub key: Key,
    pub holding_pitch: Option<Pitch>,
    pub playing_pitch: Option<Pitch>,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct State {
    holding_pitch: Option<Pitch>,
    playing_pitch: Option<Pitch>,
    pending_command: Option<Command>,
}

impl Arpeggiator {

    pub fn from_specs(specs: Specs) -> Arpeggiator {
        Arpeggiator::from_phrase(specs.key, Phrase::from_specs(specs.phrase))
    }

    pub fn from_phrase(key: Key, phrase: Phrase) -> Arpeggiator {
        Arpeggiator {
            phrase, key,
            holding_pitch: None,
            playing_pitch: None,
            pending_command: None,
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, _, _) => self.start(pitch),
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

    pub fn next(&mut self, from_measure: MeasurePosition, to_measure: MeasurePosition) -> Vec<Command> {
        match mem::replace(&mut self.pending_command, None) {
            Some(pending) => vec![pending],
            None => self.next_notes(from_measure, to_measure).iter()
                        .flat_map(|e| self.update_and_command(e)).collect()
        }
    }

    fn next_notes(&mut self, from_measure: MeasurePosition, to_measure: MeasurePosition) -> Vec<Note> {
        self.phrase.range(from_measure, to_measure)
    }

    fn update_and_command(&mut self, note: &Note) -> Vec<Command> {
        match (self.holding_pitch, self.playing_pitch) {
            (Some(holding), None) =>
                self.update_note_on(note.pitch, holding).into_iter().collect(),
            (Some(holding), Some(playing)) =>
                vec![ Some(note_off(playing)),
                      self.update_note_on(note.pitch, holding),
                ].into_iter().flatten().collect(),
            (None, Some(playing)) => {
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

    pub fn view(&self) -> View {
        View {
            phrase: self.phrase.view(),
            key: self.key,
            holding_pitch: self.holding_pitch,
            playing_pitch: self.playing_pitch,
        }
    }

    pub fn state(&self) -> State {
        State {
            holding_pitch: self.holding_pitch,
            playing_pitch: self.playing_pitch,
            pending_command: self.pending_command.clone(),
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.holding_pitch = state.holding_pitch;
        self.playing_pitch = state.playing_pitch;
        self.pending_command = state.pending_command;
    }

}

fn note_on(pitch: Pitch) -> Command {
    Command::NoteOn(pitch, 1., id(pitch))
}

fn note_off(pitch: Pitch) -> Command {
    Command::NoteOff(id(pitch))
}

use std::fmt::{Debug, Formatter};
use crate::core::tools::arpeggiator::phrase::Phrase;

impl Debug for Arpeggiator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "key: {:?}, holding: {:?}, playing: {:?}, pending: {:?}",
               self.key, self.holding_pitch, self.playing_pitch, self.pending_command)
    }
}