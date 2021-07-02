mod mix;
mod basic;
mod pulse;

use super::{Sample, Seconds, Proportion, modulated::*};
use crate::core::music_theory::Hz;
use crate::core::synth::oscillator::basic::{Sine, Square, Saw};
use crate::core::synth::oscillator::pulse::Pulse;

#[derive(Clone, PartialEq, Debug)]
pub enum Specs {
    Sine, Saw, Square, Pulse(Proportion),
    Mix {
        nvoices: usize,
        detune_amount: Hz,
        specs: Box<Specs>,
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModTarget { PulseDuty, MixThickness }

pub trait Oscillator: Modulated<ModTarget> {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample;
    fn view(&self) -> View;
}

impl dyn Oscillator {
    pub fn new(spec: &Specs) -> Box<dyn Oscillator> {
        match spec {
            Specs::Sine => Box::new(Sine),
            Specs::Square => Box::new(Square),
            Specs::Pulse(d) => Box::new(Pulse::new(*d)),
            Specs::Saw => Box::new(Saw),
            Specs::Mix {nvoices: v, detune_amount: d, specs: s} =>
                Box::new(mix::Mix::detuned(*v, *d, s)),
        }
    }
}

impl Default for Specs {
    fn default() -> Self {
        Specs::Sine
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MixVoiceView {
    pub tuning: Hz,
    pub oscillator: Box<View>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum View {
    Sine, Saw, Square, Pulse(Proportion),
    Mix {
        voices: Vec<MixVoiceView>
    }
}

impl Default for View {
    fn default() -> Self {
        View::Sine
    }
}