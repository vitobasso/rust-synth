use super::{Sample, modulated::*};
use crate::core::music_theory::Hz;

const MAX_CUTOFF: Hz = 440. * 8.;
const MAX_QFACTOR: f64 = 50.;
const MIN_QFACTOR: f64 = 1.;

pub trait Filter: Modulated<ModTarget> + Send {
    fn filter(&mut self, input: Sample) -> Sample;
    fn view(&self) -> View;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Specs { LPF, HPF, BPF, Notch, }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModTarget { Cutoff, QFactor }

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct View {
    pub cutoff: f64,
    pub resonance: f64,
}

impl dyn Filter {
    pub fn new(specs: Specs, sample_rate: Hz) -> Box<dyn Filter> {
        let filter = biquad::BiquadFilter::new(sample_rate, specs);
        Box::new(filter)
    }
}

mod biquad {

    use super::*;
    use crate::core::{synth::Sample, music_theory::Hz};
    use std::f64::consts::PI;

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
            let filter_type: Box<dyn FilterType> = match specs {
                Specs::LPF => Box::new(Lpf),
                Specs::HPF => Box::new(Hpf),
                Specs::BPF => Box::new(Bpf),
                Specs::Notch => Box::new(Notch),
            };
            BiquadFilter {
                sample_rate, filter_type,
                cutoff: ModParam::with_base(1., 0., MAX_CUTOFF),
                qfactor: ModParam::with_base(0.05, MIN_QFACTOR, MAX_QFACTOR),
                input_history: [0., 0.],
                output_history: [0., 0.],
            }
        }

        fn calculate_coefficients(&mut self) -> Coefficients {
            let cutoff = self.cutoff.calculate();
            let qfactor = self.qfactor.calculate();
            let w0 = 2. * PI * cutoff / self.sample_rate;
            let alpha = w0.sin() / (2. * qfactor);
            self.filter_type.specific_coefficients(w0, alpha)
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
                cutoff: self.cutoff.get_signal(),
                resonance: self.qfactor.get_signal(),
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

    trait FilterType : Send {
        fn specific_coefficients(&self, w0: f64, alpha: f64) -> Coefficients;
    }

    struct Lpf;
    impl FilterType for Lpf {
        fn specific_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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
    }

    struct Hpf;
    impl FilterType for Hpf {
        fn specific_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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
    }

    struct Bpf;
    impl FilterType for Bpf {
        fn specific_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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
    }

    struct Notch;
    impl FilterType for Notch {
        fn specific_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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
    }

}

impl Default for Specs {
    fn default() -> Self {
        Specs::LPF
    }
}

impl Default for View {
    fn default() -> Self {
        View {
            cutoff: 1.,
            resonance: 0.
        }
    }
}