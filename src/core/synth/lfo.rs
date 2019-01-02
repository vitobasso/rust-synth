use super::oscillator::{self, Oscillator};
use core::synth::{Seconds};
use core::music_theory::Hz;

#[derive(Copy, Clone)]
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
    oscillator: Box<Oscillator>,
    freq: Hz,
    phase: Seconds,
}

impl LFO {
    pub fn new(specs: Specs) -> LFO {
        LFO {
            oscillator: Oscillator::new(specs.oscillator),
            freq: specs.freq,
            phase: specs.phase,
        }
    }

    pub fn next(&self, clock: Seconds) -> f64 {
        self.oscillator.next_sample(clock, self.freq, self.phase)
    }
}
