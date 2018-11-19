
use super::Sample;
use std::f64::consts::PI;

pub trait Filter {
    fn filter(&mut self, cutoff: f64, q_factor: f64, input: Sample, sample_rate: f64) -> Sample;
}

/// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
pub struct BiquadFilter {
    input_history: [Sample;2],
    output_history: [Sample;2],
    coefficients: Box<Fn(f64, f64) -> Coefficients>,
}
pub struct Coefficients {
    b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
}

impl Filter for BiquadFilter {
    fn filter(&mut self, cutoff: f64, q_factor: f64, input: Sample, sample_rate: f64) -> Sample {
        let w0 = 2. * PI * cutoff / sample_rate;
        let alpha = w0.sin() / (2. * q_factor);
        let coef = (self.coefficients)(w0, alpha);
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
    pub fn new(coefficients: Box<Fn(f64, f64) -> Coefficients>) -> Self {
        BiquadFilter {
            input_history: [0., 0.],
            output_history: [0., 0.],
            coefficients,
        }
    }
    pub fn lpf() -> Self {
        let coef = |w0: f64, alpha: f64| {
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
        BiquadFilter::new(Box::new(coef))
    }
    pub fn hpf() -> Self {
        let coef = |w0: f64, alpha: f64| {
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
        BiquadFilter::new(Box::new(coef))
    }
    pub fn bpf() -> Self {
        let coef = |w0: f64, alpha: f64| {
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
        BiquadFilter::new(Box::new(coef))
    }
    pub fn notch() -> Self {
        let coef = |w0: f64, alpha: f64| {
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
        BiquadFilter::new(Box::new(coef))
    }
}