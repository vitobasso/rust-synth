use std::collections::HashMap;
use crate::core::{
    music_theory::{Hz, pitch::Pitch},
    synth::{Sample, Velocity, instrument::{self, Instrument}, oscillator},
};

///
/// Interprets commands by translating to synth::instrument method calls
///

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl State {

    pub fn new(specs: instrument::Specs, sample_rate: Hz) -> State {
        State {
            sample_rate,
            instrument: Instrument::new(specs, sample_rate),
            holding_notes: HashMap::new(),
        }
    }

    pub fn with_default_specs(sample_rate: Hz) -> State {
        State::new(instrument::Specs::default(), sample_rate)
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, velocity, id) => self.handle_note_on(pitch, velocity, id),
            Command::NoteOff(id) => self.handle_note_off(id),
            Command::ModXY(x, y) => self.instrument.set_xy_params(x, y),
            Command::SetPatch(specs) => self.set_instrument(specs),
        }
    }

    pub fn next_sample(&mut self) -> Sample {
        self.instrument.next_sample()
    }

    pub fn view(&self) -> instrument::View {
        self.instrument.view()
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

    fn set_instrument(&mut self, specs: instrument::Specs) {
        self.instrument = Instrument::new(specs, self.sample_rate);
    }

    pub fn set_oscillator(&mut self, specs: oscillator::Specs) {
        self.instrument.set_oscillator(specs);
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
