mod biquad;

use super::{Sample, modulated::*};
use crate::core::music_theory::Hz;

const MAX_CUTOFF: Hz = 440. * 8.;
const MAX_QFACTOR: f64 = 50.;
const MIN_QFACTOR: f64 = 1.;

pub trait Filter: Modulated<ModTarget> {
    fn filter(&mut self, input: Sample) -> Sample;
    fn view(&self) -> View;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Specs { LPF, HPF, BPF, Notch, }

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
        Specs::LPF
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct View {
    pub cutoff: f64,
    pub resonance: f64,
    pub filter_type: Specs,
}

impl Default for View {
    fn default() -> Self {
        View {
            cutoff: 1.,
            resonance: 0.,
            filter_type: Specs::LPF
        }
    }
}