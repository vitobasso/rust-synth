
use super::*;
use crate::core::{synth::Sample, music_theory::Hz};
use std::f64::consts::PI;
use crate::core::synth::filter;

/// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
pub(super) struct BiquadFilter{
    sample_rate: Hz,
    cutoff: ModParam,
    qfactor: ModParam,
    filter_type: Box<dyn FilterType>,
    input_history: [Sample;2],
    output_history: [Sample;2],
}

impl BiquadFilter {
    pub(super) fn new(sample_rate: Hz, specs: Specs) -> BiquadFilter {
        assert!(sample_rate > 0., "sample_rate was: {}", sample_rate);
        let filter_type: Box<dyn FilterType> = match specs.filter_type {
            TypeSpec::LPF => Box::new(Lpf),
            TypeSpec::HPF => Box::new(Hpf),
            TypeSpec::BPF => Box::new(Bpf),
            TypeSpec::Notch => Box::new(Notch),
        };
        BiquadFilter {
            sample_rate, filter_type,
            cutoff: ModParam::with_base(specs.cutoff, MIN_CUTOFF, MAX_CUTOFF),
            qfactor: ModParam::with_base(specs.resonance, MIN_QFACTOR, MAX_QFACTOR),
            input_history: [0., 0.],
            output_history: [0., 0.],
        }
    }

    fn calculate_coefficients(&mut self) -> Coefficients {
        let cutoff = self.cutoff.calculate();
        let qfactor = self.qfactor.calculate();
        let w0 = 2. * PI * cutoff / self.sample_rate;
        let alpha = w0.sin() / (2. * qfactor);
        self.filter_type.coefficients(w0, alpha)
    }
}

impl Filter for BiquadFilter {
    fn filter(&mut self, input: Sample) -> Sample {
        let coef = &self.calculate_coefficients();
        let a0 = coef.a0;
        let output = (coef.b0/a0) * input
            + (coef.b1/a0) * self.input_history[1]  + (coef.b2/a0) * self.input_history[0]
            - (coef.a1/a0) * self.output_history[1] - (coef.a2/a0) * self.output_history[0];

        self.input_history  = [self.input_history[1], input];
        self.output_history = [self.output_history[1], output];

        output
    }

    fn view(&self) -> View {
        View {
            cutoff: self.cutoff.normalized(),
            resonance: self.qfactor.normalized(),
            filter_type: self.filter_type.spec(),
        }
    }

    fn state(&self) -> filter::State {
        let state = State {
            cutoff: self.cutoff.mod_signal,
            resonance: self.qfactor.mod_signal,
            input_history: self.input_history,
            output_history: self.output_history,
        };
        filter::State::Biquad(state)
    }

    fn set_state(&mut self, state: filter::State) {
        match state {
            filter::State::Biquad(s) => {
                self.cutoff.mod_signal = s.cutoff;
                self.qfactor.mod_signal = s.resonance;
                self.input_history = s.input_history;
                self.output_history = s.output_history;
            }
        }
    }
}

impl Modulated<ModTarget> for BiquadFilter {
    fn mod_param(&mut self, target: ModTarget) -> Option<&mut ModParam> {
        match target {
            ModTarget::Cutoff => Some(&mut self.cutoff),
            ModTarget::QFactor => Some(&mut self.qfactor),
        }
    }
}

struct Coefficients {
    b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
}

trait FilterType {
    fn coefficients(&self, w0: f64, alpha: f64) -> Coefficients;
    fn spec(&self) -> TypeSpec;
}

struct Lpf;
impl FilterType for Lpf {
    fn coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
        let cos_w0 = w0.cos();
        Coefficients {
            b0: (1. - cos_w0) / 2.,
            b1:  1. - cos_w0,
            b2: (1. - cos_w0) / 2.,
            a0:  1. + alpha,
            a1: -2. * cos_w0,
            a2:  1. - alpha,
        }
    }

    fn spec(&self) -> TypeSpec {
        TypeSpec::LPF
    }
}

struct Hpf;
impl FilterType for Hpf {
    fn coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
        let cos_w0 = w0.cos();
        Coefficients{
            b0:  (1. + cos_w0)/2.,
            b1: -(1. + cos_w0),
            b2:  (1. + cos_w0)/2.,
            a0:   1. + alpha,
            a1:  -2. * cos_w0,
            a2:   1. - alpha,
        }
    }

    fn spec(&self) -> TypeSpec {
        TypeSpec::HPF
    }
}

struct Bpf;
impl FilterType for Bpf {
    fn coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        Coefficients{
            b0:   sin_w0/2.,
            b1:   0.,
            b2:  -sin_w0/2.,
            a0:   1. + alpha,
            a1:  -2. * cos_w0,
            a2:   1. - alpha,
        }
    }

    fn spec(&self) -> TypeSpec {
        TypeSpec::BPF
    }
}

struct Notch;
impl FilterType for Notch {
    fn coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
        let cos_w0 = w0.cos();
        Coefficients{
            b0:   1.,
            b1:  -2. * cos_w0,
            b2:   1.,
            a0:   1. + alpha,
            a1:  -2. * cos_w0,
            a2:   1. - alpha,
        }
    }

    fn spec(&self) -> TypeSpec {
        TypeSpec::Notch
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct State {
    pub cutoff: f64,
    pub resonance: f64,
    pub input_history: [Sample;2],
    pub output_history: [Sample;2],
}