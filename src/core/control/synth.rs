use std::collections::HashMap;
use crate::core::{
    music_theory::{Hz, pitch::Pitch},
    synth::{Sample, Velocity, instrument::{self, Instrument}},
};

///
/// Interprets commands by translating to synth::instrument method calls
///

#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    NoteOn(Pitch, Velocity, Id), NoteOff(Id),
    ModXY(f64, f64),
    SetPatch(instrument::Specs),
}

pub struct State {
    sample_rate: Hz,
    instrument: Instrument,
    holding_notes: HashMap<Id, Pitch>,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub instrument: instrument::View,
    pub holding_notes: HashMap<Id, Pitch>,
}

impl State {

    pub fn new(sample_rate: Hz) -> State {
        let specs = instrument::Specs::default();
        State {
            sample_rate,
            instrument: Instrument::new(specs, sample_rate),
            holding_notes: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, velocity, id) => self.handle_note_on(pitch, velocity, id),
            Command::NoteOff(id) => self.handle_note_off(id),
            Command::ModXY(x, y) => self.instrument.set_xy_params(x, y),
            Command::SetPatch(specs) => self.set_specs(specs),
        }
    }

    pub fn next_sample(&mut self) -> Sample {
        self.instrument.next_sample()
    }

    pub fn view(&self) -> View {
        View {
            instrument: self.instrument.view(),
            holding_notes: self.holding_notes.clone(),
        }
    }

    fn handle_note_on(&mut self, pitch: Pitch, velocity: Velocity, id: Id) {
        if self.holding_notes.insert(id, pitch).is_none() {
            self.instrument.hold(pitch, velocity)
        }
    }

    fn handle_note_off(&mut self, id: Id) {
        if let Some(remembered_pitch) = self.holding_notes.remove(&id) {
            self.instrument.release(remembered_pitch)
        }
    }

    fn set_specs(&mut self, specs: instrument::Specs) {
        let state = self.instrument.get_state();
        self.instrument = Instrument::new(specs, self.sample_rate);
        self.instrument.set_state(state);
    }

    pub fn get_specs(&self) -> instrument::Specs {
        self.instrument.get_specs()
    }
}

pub type Discriminator = u8;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Id {
    pub pitch: Pitch,
    pub discriminator: Option<Discriminator>,
}

use std::fmt::{Debug, Formatter};
impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id( {:?}, {:?} )", self.pitch, self.discriminator)
    }
}

pub const fn id(pitch: Pitch) -> Id {
    Id { pitch, discriminator: None }
}

pub const fn id_discr(pitch: Pitch, discriminator: Discriminator) -> Id {
    Id { pitch, discriminator: Some(discriminator) }
}
