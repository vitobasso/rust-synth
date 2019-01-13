use std::collections::HashMap;
use crate::core::{
    music_theory::{Hz, pitch::Pitch},
    synth::{Sample, instrument::{self, Instrument}, oscillator},
};

#[derive(Clone, Copy, Debug)]
pub enum Command {
    NoteOn(Pitch, Id), NoteOff(Id),
    ModXY(f64, f64),
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
        State::new(default_instrument_specs(), sample_rate)
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, id) => self.handle_note_on(pitch, id),
            Command::NoteOff(id) => self.handle_note_off(id),
            Command::ModXY(x, y) => self.instrument.set_xy_params(x, y),
        }
    }

    pub fn next_sample(&mut self) -> Sample {
        self.instrument.next_sample()
    }

    fn handle_note_on(&mut self, pitch: Pitch, id: Id) {
        if self.holding_notes.insert(id, pitch).is_none() {
            self.instrument.hold(pitch)
        }
    }

    fn handle_note_off(&mut self, id: Id) {
        if let Some(remembered_pitch) = self.holding_notes.remove(&id) {
            self.instrument.release(remembered_pitch)
        }
    }

    pub fn set_instrument(&mut self, specs: instrument::Specs) {
        self.instrument = Instrument::new(specs, self.sample_rate);
    }

    pub fn set_oscillator(&mut self, specs: oscillator::Specs) {
        self.instrument.set_oscillator(specs);
    }
}

use crate::core::synth::{
    instrument::ModTarget::*, oscillator::Specs::*, filter::ModTarget::*,
    lfo, builder::Builder,
};
fn default_instrument_specs() -> instrument::Specs {
    Builder::osc(Supersaw { nvoices: 8, detune_amount: 3.})
        .lfo(lfo::Specs::simple(0.1), Filter(Cutoff), 0.8).build()
}

pub type Discriminator = u8;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Id {
    pub pitch: Pitch,
    pub discriminator: Option<Discriminator>,
}

pub fn id(pitch: Pitch) -> Id {
    Id { pitch, discriminator: None }
}

pub fn id_discr(pitch: Pitch, discriminator: Discriminator) -> Id {
    Id { pitch, discriminator: Some(discriminator) }
}
