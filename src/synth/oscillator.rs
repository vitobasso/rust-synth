extern crate rand;

use super::Sample;
use self::rand::{Rng, ThreadRng};
use std::f64::consts::PI;

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
