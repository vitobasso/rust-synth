
use super::{Sample, Hz};
use std::f64::consts::PI;

const MAX_CUTOFF: Hz = 440. * 8.;
const MAX_Q_FACTOR: f64 = 50.;

pub trait Filter {
    fn set_params(&mut self, cutoff: Hz, q_factor: f64);
    fn filter(&mut self, input: Sample) -> Sample;
}

/// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
pub struct BiquadFilter {
    sample_rate: Hz,
    input_history: [Sample;2],
    output_history: [Sample;2],
    coefficients: Coefficients,
    calculate: Box<Fn(f64, f64) -> Coefficients>,
}
pub struct Coefficients {
    b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
}

impl Filter for BiquadFilter {
    fn set_params(&mut self, cutoff: Hz, q_factor: f64) {
        assert!(cutoff >= 0. && cutoff <= 1., "cutoff was: {}", cutoff);
        assert!(q_factor >= 0. && q_factor <= 1., "q_factor was: {}", q_factor);
        let scaled_cutoff = cutoff * MAX_CUTOFF;
        let scaled_q_factor = q_factor * MAX_Q_FACTOR;
        let w0 = 2. * PI * scaled_cutoff / self.sample_rate;
        let alpha = w0.sin() / (2. * scaled_q_factor);
        self.coefficients = (self.calculate)(w0, alpha);
    }
    fn filter(&mut self, input: Sample) -> Sample {
        let coef = &self.coefficients;
        let a0 = coef.a0;
        let output = (coef.b0/a0) * input
            + (coef.b1/a0) * self.input_history[1]  + (coef.b2/a0) * self.input_history[0]
            - (coef.a1/a0) * self.output_history[1] - (coef.a2/a0) * self.output_history[0];

        self.input_history  = [self.input_history[1], input];
        self.output_history = [self.output_history[1], output];

        output
    }
}

impl BiquadFilter {
    pub fn new(sample_rate: Hz, calculate: Box<Fn(f64, f64) -> Coefficients>) -> Self {
        assert!(sample_rate > 0., "sample_rate was: {}", sample_rate);
        BiquadFilter {
            sample_rate,
            input_history: [0., 0.],
            output_history: [0., 0.],
            coefficients: Coefficients{ b0: 0., b1: 0., b2: 0., a0: 0., a1: 0., a2: 0. },
            calculate,
        }
    }
    pub fn lpf(sample_rate: Hz) -> Self {
        let calculate = |w0: f64, alpha: f64| {
            let cos_w0 = w0.cos();
            Coefficients {
                b0: (1. - cos_w0) / 2.,
                b1:  1. - cos_w0,
                b2: (1. - cos_w0) / 2.,
                a0:  1. + alpha,
                a1: -2. * cos_w0,
                a2:  1. - alpha,
            }
        };
        BiquadFilter::new(sample_rate, Box::new(calculate))
    }
    pub fn hpf(sample_rate: Hz) -> Self {
        let calculate = |w0: f64, alpha: f64| {
            let cos_w0 = w0.cos();
            Coefficients{
                b0:  (1. + cos_w0)/2.,
                b1: -(1. + cos_w0),
                b2:  (1. + cos_w0)/2.,
                a0:   1. + alpha,
                a1:  -2. * cos_w0,
                a2:   1. - alpha,
            }
        };
        BiquadFilter::new(sample_rate, Box::new(calculate))
    }
    pub fn bpf(sample_rate: Hz) -> Self {
        let calculate = |w0: f64, alpha: f64| {
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
        };
        BiquadFilter::new(sample_rate, Box::new(calculate))
    }
    pub fn notch(sample_rate: Hz) -> Self {
        let calculate = |w0: f64, alpha: f64| {
            let cos_w0 = w0.cos();
            Coefficients{
                b0:   1.,
                b1:  -2. * cos_w0,
                b2:   1.,
                a0:   1. + alpha,
                a1:  -2. * cos_w0,
                a2:   1. - alpha,
            }
        };
        BiquadFilter::new(sample_rate, Box::new(calculate))
    }
}