extern crate rand;

use std::f64::consts::PI;
use pitches::Pitch;
use self::rand::{Rng, ThreadRng};

type Sample = f64;

pub trait Osc {
    fn next_sample(&self, clock: f64, freq: f64, phase: f64) -> Sample;
}

pub struct Sine;
impl Osc for Sine {
    fn next_sample(&self, clock: f64, freq: f64, phase: f64) -> Sample {
        ((clock + phase) * freq * 2. * PI).sin()
    }
}

pub struct Saw;
impl Osc for Saw {
    fn next_sample(&self, clock: f64, freq: f64, phase: f64) -> Sample {
        (((clock + phase) * freq) % 1.)
    }
}

pub struct StatefulSaw { pub detune: f64 }
impl Osc for StatefulSaw {
    fn next_sample(&self, clock: f64, freq: f64, phase: f64) -> f64 {
        let final_freq = freq + self.detune;
        Saw.next_sample(clock, final_freq, phase)
    }
}

pub struct Mix { pub voices: Vec<Box<Osc>> }
impl Osc for Mix {
    fn next_sample(&self, clock: f64, freq: f64, phase: f64) -> Sample {
        let sum: f64 = self.voices.iter()
            .map(|o| o.next_sample(clock, freq, phase))
            .sum();
        sum / self.voices.len() as f64
    }
}
impl Mix {
    pub fn supersaw(n_voices: u16, detune_amount: f64) -> Mix {
        let mut rng = rand::thread_rng();
        fn random_around_zero(rng: &mut ThreadRng, amount: f64) -> f64 {
            rng.gen_range(-amount, amount)
        }

        let mut saws: Vec<Box<Osc>> = Vec::new();
        for _ in 0..n_voices {
            let saw = StatefulSaw { detune: random_around_zero(&mut rng, detune_amount) };
            saws.push(Box::new(saw))
        }
        Mix { voices: saws }
    }
}

pub trait Filter {
    fn filter(&mut self, cutoff: f64, q_factor: f64, input: Sample, sample_rate: f64) -> Sample;
}

pub struct BiquadFilter {
    input_history: [Sample;2],
    output_history: [Sample;2],
    coefficients: Box<Fn(f64, f64) -> Coefficients>,
}
pub struct Coefficients {
    b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64,
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
                b1: 1. - cos_w0,
                b2: (1. - cos_w0) / 2.,
                a0: 1. + alpha,
                a1: -2. * cos_w0,
                a2: 1. - alpha,
            }
        };
        BiquadFilter::new(Box::new(coef))
    }
    pub fn hpf() -> Self {
        let coef = |w0: f64, alpha: f64| {
            let cos_w0 = w0.cos();
            Coefficients{
                b0: (1. + cos_w0)/2.,
                b1: -(1. + cos_w0),
                b2: (1. + cos_w0)/2.,
                a0: 1. + alpha,
                a1: -2. * cos_w0,
                a2: 1. - alpha,
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
impl Filter for BiquadFilter {
    /// http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
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

pub trait WaveGen {
    fn next_sample(&mut self, clock: f64, sample_rate: f64) -> Sample;
}

pub struct Instrument {
    pub pitch: Pitch,
    pub oscilator: Box<Osc>,
    pub filter: Box<Filter>,
    pub mod_param_1: f64,
    pub mod_param_2: f64,
}

impl WaveGen for Instrument {
    fn next_sample(&mut self, clock: f64, sample_rate: f64) -> Sample {
        let raw = self.oscilator.next_sample(clock, self.pitch.freq(), 0.);
        self.filter.filter(self.mod_param_1, self.mod_param_2, raw, sample_rate)
    }
}
