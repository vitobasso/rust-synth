mod biquad;

use super::{Sample, modulated::*};
use crate::core::music_theory::Hz;

const MAX_CUTOFF: Hz = 440. * 32.;
const MIN_CUTOFF: Hz = 0.;
const MAX_QFACTOR: f64 = 50.;
const MIN_QFACTOR: f64 = 1.;

pub trait Filter: Modulated<ModTarget> {
    fn filter(&mut self, input: Sample) -> Sample;
    fn view(&self) -> View;
    fn state(&self) -> State;
    fn set_state(&mut self, state: State);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Specs {
    pub filter_type: TypeSpec,
    pub cutoff: f64,
    pub resonance: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TypeSpec { LPF, HPF, BPF, Notch }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModTarget { Cutoff, QFactor }

impl dyn Filter {
    pub fn new(specs: Specs, sample_rate: Hz) -> Box<dyn Filter> {
        let filter = biquad::BiquadFilter::new(sample_rate, specs);
        Box::new(filter)
    }
}

impl Default for Specs {
    fn default() -> Self {
        Specs {
            filter_type: TypeSpec::LPF,
            cutoff: 1.,
            resonance: 0.05,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct View { //TODO remove if identical to Specs?
    pub cutoff: f64,
    pub resonance: f64,
    pub filter_type: TypeSpec,
}

impl Default for View {
    fn default() -> Self {
        View {
            cutoff: 1.,
            resonance: 0.,
            filter_type: TypeSpec::LPF
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Biquad(biquad::State)
}