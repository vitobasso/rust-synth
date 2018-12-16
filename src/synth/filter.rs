
use super::{Sample, Hz};
use std::f64::consts::PI;

const MAX_CUTOFF: Hz = 440. * 8.;
const MAX_Q_FACTOR: f64 = 50.;

#[derive(Clone, Copy)]
pub enum Specs { LPF, HPF, BPF, Notch, }

#[derive(Clone, Copy)]
pub struct Params { pub cutoff: Hz, pub q_factor: f64 }

pub trait Filter {
    fn set_params(&mut self, params: Params);
    fn filter(&mut self, input: Sample) -> Sample;
}
impl Filter {
    pub fn new(specs: Specs, sample_rate: Hz) -> Box<Filter> {
        match specs {
            Specs::LPF => Box::new(BiquadFilter::lpf(sample_rate)),
            Specs::HPF => Box::new(BiquadFilter::hpf(sample_rate)),
            Specs::BPF => Box::new(BiquadFilter::bpf(sample_rate)),
            Specs::Notch => Box::new(BiquadFilter::notch(sample_rate)),
        }
    }
}

/// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
pub struct BiquadFilter {
    sample_rate: Hz,
    input_history: [Sample;2],
    output_history: [Sample;2],
    coefficients: Coefficients,
    calculate: Box<Fn(f64, f64) -> Coefficients>,
}
#[derive(Debug)]
pub struct Coefficients {
    b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
}

impl Filter for BiquadFilter {
    fn set_params(&mut self, params: Params) {
        let cutoff = params.cutoff;
        let q_factor = params.q_factor;
        assert!(cutoff >= 0. && cutoff <= 1., "cutoff was: {}", cutoff);
        assert!(q_factor > 0. && q_factor <= 1., "q_factor was: {}", q_factor);
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
    fn new(sample_rate: Hz, calculate: Box<Fn(f64, f64) -> Coefficients>) -> Self {
        assert!(sample_rate > 0., "sample_rate was: {}", sample_rate);
        let mut filter = BiquadFilter {
            sample_rate,
            input_history: [0., 0.],
            output_history: [0., 0.],
            coefficients: Coefficients{ b0: 0., b1: 0., b2: 0., a0: 0., a1: 0., a2: 0. },
            calculate,
        };
        let init_params = Params{cutoff: 1., q_factor: 0.05};
        filter.set_params(init_params);
        filter
    }
    fn lpf(sample_rate: Hz) -> Self {
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
    fn hpf(sample_rate: Hz) -> Self {
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
    fn bpf(sample_rate: Hz) -> Self {
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
    fn notch(sample_rate: Hz) -> Self {
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

//TODO test invalid cutoff & q_factor values