use super::oscillator::{self, Oscillator};
use crate::core::synth::Seconds;
use crate::core::music_theory::Hz;

#[derive(Clone, PartialEq, Debug)]
pub struct Specs {
    oscillator: oscillator::Specs,
    freq: Hz,
    phase: Seconds,
}
impl Specs {
    pub fn simple(freq: Hz) -> Specs {
        Specs { freq, oscillator: oscillator::Specs::Sine, phase: 0. }
    }
}

pub struct LFO {
    oscillator: Box<dyn Oscillator>,
    freq: Hz,
    phase: Seconds,
}

impl LFO {
    pub fn new(specs: Specs) -> LFO {
        LFO {
            oscillator: <dyn Oscillator>::new(&specs.oscillator),
            freq: specs.freq,
            phase: specs.phase,
        }
    }

    pub fn next(&self, clock: Seconds) -> f64 {
        self.oscillator.next_sample(clock, self.freq, self.phase)
    }

    pub fn view(&self) -> View {
        View {
            oscillator: self.oscillator.view(),
            freq: self.freq,
            phase: self.phase,
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    oscillator: oscillator::View,
    freq: Hz,
    phase: Seconds,
}
