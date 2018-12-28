use super::Sample;
use core::music_theory::Hz;

const MAX_CUTOFF: Hz = 440. * 8.;
const MAX_QFACTOR: f64 = 50.;
const MIN_QFACTOR: f64 = 1.;

pub trait Filter {
    fn filter(&mut self, input: Sample) -> Sample;
    fn modulate(&mut self, modulation: Modulation, value: f64);
}

#[derive(Clone, Copy)]
pub enum Specs { LPF, HPF, BPF, Notch, }

#[derive(Copy, Clone)]
pub enum Modulation { Cutoff, QFactor }

impl Filter {
    pub fn new(specs: Specs, sample_rate: Hz) -> Box<Filter> {
        let filter = biquad::BiquadFilter::new(sample_rate, specs);
        Box::new(filter)
    }
}

mod biquad {

    use super::*;
    use core::{synth::Sample, music_theory::Hz};
    use std::f64::consts::PI;

    /// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
    pub(super) struct BiquadFilter{
        parameters: Parameters,
        input_history: [Sample;2],
        output_history: [Sample;2],
    }

    impl BiquadFilter {
        pub(super) fn new(sample_rate: Hz, specs: Specs) -> BiquadFilter {
            assert!(sample_rate > 0., "sample_rate was: {}", sample_rate);
            let filter_type: Box<FilterType> = match specs {
                Specs::LPF => Box::new(LPF),
                Specs::HPF => Box::new(HPF),
                Specs::BPF => Box::new(BPF),
                Specs::Notch => Box::new(Notch),
            };
            BiquadFilter {
                parameters: Parameters::new(sample_rate, filter_type),
                input_history: [0., 0.],
                output_history: [0., 0.],
            }
        }

        fn set_cutoff(&mut self, value: f64) {
            let normalized = value.powi(2).max(0.).min(1.);
            self.parameters.set_cutoff(normalized);
        }

        fn set_qfactor(&mut self, value: f64) {
            let normalized = value.powi(2).max(0.).min(1.);
            self.parameters.set_qfactor(normalized);
        }

    }

    impl Filter for BiquadFilter {

        fn filter(&mut self, input: Sample) -> Sample {
            let coef = &self.parameters.coefficients;
            let a0 = coef.a0;
            let output = (coef.b0/a0) * input
                + (coef.b1/a0) * self.input_history[1]  + (coef.b2/a0) * self.input_history[0]
                - (coef.a1/a0) * self.output_history[1] - (coef.a2/a0) * self.output_history[0];

            self.input_history  = [self.input_history[1], input];
            self.output_history = [self.output_history[1], output];

            output
        }

        fn modulate(&mut self, modulation: Modulation, value: f64) {
            match modulation {
                Modulation::Cutoff => self.set_cutoff(value),
                Modulation::QFactor => self.set_qfactor(value),
            }
        }
    }

    struct Parameters{
        sample_rate: Hz,
        cutoff: Hz,
        qfactor: f64,
        filter_type: Box<FilterType>,
        coefficients: Coefficients,
    }

    struct Coefficients {
        b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
    }

    impl Parameters {

        fn new(sample_rate: Hz, filter_type: Box<FilterType>) -> Parameters {
            let cutoff = 1.;
            let qfactor = 0.05;
            let coefficients = calculate(sample_rate, cutoff, qfactor, &filter_type);
            Parameters { sample_rate, cutoff, qfactor, filter_type, coefficients }
        }

        fn set_cutoff(&mut self, value: f64) {
            self.cutoff = value;
            self.recalculate();
        }

        fn set_qfactor(&mut self, value: f64) {
            self.qfactor = value;
            self.recalculate();
        }

        fn recalculate(&mut self) {
            self.coefficients = calculate(self.sample_rate, self.cutoff, self.qfactor, &self.filter_type);
        }

    }

    fn calculate(sample_rate: Hz, cutoff: Hz, qfactor: f64, filter_type: &Box<FilterType>) -> Coefficients {
        let scaled_cutoff = cutoff * MAX_CUTOFF;
        let scaled_qfactor = (qfactor * MAX_QFACTOR).max(MIN_QFACTOR);
        let w0 = 2. * PI * scaled_cutoff / sample_rate;
        let alpha = w0.sin() / (2. * scaled_qfactor);
        filter_type.calculate_coefficients(w0, alpha)
    }

    trait FilterType {
        fn calculate_coefficients(&self, w0: f64, alpha: f64) -> Coefficients;
    }

    struct LPF;
    impl FilterType for LPF {
        fn calculate_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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

    struct HPF;
    impl FilterType for HPF {
        fn calculate_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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

    struct BPF;
    impl FilterType for BPF {
        fn calculate_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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
        fn calculate_coefficients(&self, w0: f64, alpha: f64) -> Coefficients {
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