use std::collections::HashMap;
use core::{
    music_theory::{Hz, Semitones, pitch::{Pitch, PitchClass}, diatonic_scale::Key},
    synth::{Sample, instrument::{self, Instrument}, oscillator},
};

pub type Id = u32;

#[derive(Debug)]
pub enum Command {
    NoteOn(Pitch, Id), NoteOff(Pitch, Id),
    ModXY(f64, f64),
    ShiftPitch(Semitones), ShiftKeyboard(Semitones), TransposeKey(Semitones),
}

pub struct State {
    sample_rate: Hz,
    instrument: Instrument,
    key: Key,
    pitch_shift: Semitones,
    holding_notes: HashMap<(Pitch, Id), Pitch>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            instrument: Instrument::new(default_instrument_specs(), sample_rate),
            key: PitchClass::C,
            pitch_shift: 0,
            holding_notes: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, id) => self.handle_note_on(pitch, id),
            Command::NoteOff(pitch, id) => self.handle_note_off(pitch, id),
            Command::ModXY(x, y) => self.instrument.set_xy_params(x, y),
            Command::ShiftPitch(n) => self.pitch_shift = self.pitch_shift + n,
            Command::ShiftKeyboard(n) => {
                self.key = self.key + n;
                self.pitch_shift = self.pitch_shift - n;
            }
            Command::TransposeKey(n) => self.key = self.key.circle_of_fifths(n),
        }
    }

    pub fn next_sample(&mut self) -> Sample {
        self.instrument.next_sample()
    }

    fn handle_note_on(&mut self, input_pitch: Pitch, id: Id) {
        let transposed_pitch = self.transpose(input_pitch);
        if let None = self.holding_notes.insert((input_pitch, id), transposed_pitch) {
            self.instrument.hold(transposed_pitch)
        }
    }

    fn handle_note_off(&mut self, input_pitch: Pitch, id: Id) {
        if let Some(remembered_pitch) = self.holding_notes.remove(&(input_pitch, id)) {
            self.instrument.release(remembered_pitch)
        }
    }

    fn transpose(&self, pitch: Pitch) -> Pitch {
        let transposed = PitchClass::C.transpose_to(self.key, pitch)
            .expect(&format!("Failed to transpose: {:?}", pitch));
        transposed + self.pitch_shift
    }

    pub fn set_instrument(&mut self, specs: instrument::Specs) {
        self.instrument = Instrument::new(specs, self.sample_rate);
    }

    pub fn set_oscillator(&mut self, specs: oscillator::Specs) {
        self.instrument.set_oscillator(specs);
    }
}

use core::synth::{
    instrument::ModTarget::*, oscillator::Specs::*, filter::ModTarget::*,
    lfo, builder::Builder,
};
fn default_instrument_specs() -> instrument::Specs {
    Builder::osc(Supersaw { nvoices: 8, detune_amount: 3.})
        .lfo(lfo::Specs::simple(0.1), Filter(Cutoff), 0.8).build()
}