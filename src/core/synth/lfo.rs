use super::oscillator::{self, Oscillator, Basic::Sine};
use crate::core::synth::{Seconds, Proportion};
use crate::core::music_theory::Hz;
use crate::core::synth::instrument::ModTarget;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Specs {
    pub oscillator: oscillator::Specs,
    pub freq: Hz,
    pub phase: Seconds,
    pub amount: Proportion,
    pub target: ModTarget,
}
impl Specs {
    pub fn simple(freq: Hz, amount: Proportion, target: ModTarget) -> Specs {
        Specs { freq, oscillator: oscillator::Specs::Basic(Sine), phase: 0., amount, target }
    }
}

pub struct LFO {
    oscillator: Box<dyn Oscillator>,
    freq: Hz,
    phase: Seconds,
    amount: Proportion,
    pub target: ModTarget,
}

impl LFO {
    pub fn new(specs: Specs) -> LFO {
        LFO {
            oscillator: <dyn Oscillator>::new(&specs.oscillator),
            freq: specs.freq,
            phase: specs.phase,
            amount: specs.amount,
            target: specs.target,
        }
    }

    pub fn next(&self, clock: Seconds) -> f64 {
        let around_zero = self.oscillator.next_sample(clock, self.freq, self.phase);
        let positive = (around_zero + 1.) / 2.;
        positive * self.amount
    }

    pub fn view(&self) -> View {
        View {
            oscillator: self.oscillator.view(),
            freq: self.freq,
            phase: self.phase,
            amount: self.amount,
            target: self.target,
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub oscillator: oscillator::View,
    pub freq: Hz,
    pub phase: Seconds,
    pub amount: Proportion,
    pub target: ModTarget,
}
