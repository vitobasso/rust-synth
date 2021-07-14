mod mix;
mod basic;
mod pulse;

use super::{Sample, Seconds, Proportion, modulated::*};
use crate::core::music_theory::Hz;
use crate::core::synth::oscillator::basic::{Sine, Square, Saw};
use crate::core::synth::oscillator::pulse::Pulse;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Basic {
    Sine, Saw, Square
}

#[derive(Clone, PartialEq, Debug)]
pub enum Specs {
    Basic(Basic),
    Pulse(Proportion),
    Mix {
        n_voices: usize,
        detune_amount: Hz,
        specs: Basic,
        random_seed: u64,
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModTarget { PulseDuty }

pub trait Oscillator: Modulated<ModTarget> {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample;
    fn view(&self) -> View;
}

impl dyn Oscillator {
    pub fn new(spec: &Specs) -> Box<dyn Oscillator> {
        match spec {
            Specs::Basic(Basic::Sine) => Box::new(Sine),
            Specs::Basic(Basic::Square) => Box::new(Square),
            Specs::Basic(Basic::Saw) => Box::new(Saw),
            Specs::Pulse(duty_cycle) => Box::new(Pulse::new(*duty_cycle)),
            Specs::Mix { n_voices, detune_amount, specs, random_seed } =>
                Box::new(mix::Mix::detuned(*n_voices, *detune_amount, *specs, *random_seed)),
        }
    }
}

impl Default for Specs {
    fn default() -> Self {
        Specs::Basic(Basic::Sine)
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