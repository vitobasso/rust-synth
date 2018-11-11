extern crate rand;

use std::f32::consts::PI;
use pitches::Pitch;
use self::rand::{Rng, ThreadRng};

type Sample = f32;

pub trait Osc {
    fn next_sample(&self, clock: f32, freq: f32, phase: f32) -> Sample;
}

pub struct Sine;
impl Osc for Sine {
    fn next_sample(&self, clock: f32, freq: f32, phase: f32) -> Sample {
        ((clock + phase) * freq * 2.0 * PI).sin()
    }
}

pub struct Saw;
impl Osc for Saw {
    fn next_sample(&self, clock: f32, freq: f32, phase: f32) -> Sample {
        (((clock + phase) * freq) % 1.0)
    }
}

pub struct StatefulSaw { pub detune: f32 }
impl Osc for StatefulSaw {
    fn next_sample(&self, clock: f32, freq: f32, phase: f32) -> f32 {
        let final_freq = freq + self.detune;
        Saw.next_sample(clock, final_freq, phase)
    }
}

pub struct Mix { pub voices: Vec<Box<Osc>> }
impl Osc for Mix {
    fn next_sample(&self, clock: f32, freq: f32, phase: f32) -> Sample {
        let sum: f32 = self.voices.iter()
            .map(|o| o.next_sample(clock, freq, phase))
            .sum();
        sum / self.voices.len() as f32
    }
}
impl Mix {
    pub fn supersaw(n_voices: u16, detune_amount: f32) -> Mix {
        let mut rng = rand::thread_rng();
        fn random_around_zero(rng: &mut ThreadRng, amount: f32) -> f32 {
            rng.gen_range(-amount as f64, amount as f64) as f32
        }

        let mut saws: Vec<Box<Osc>> = Vec::new();
        for _ in 0..n_voices {
            let saw = StatefulSaw { detune: random_around_zero(&mut rng, detune_amount as f32) };
            saws.push(Box::new(saw))
        }
        Mix { voices: saws }
    }
}


pub trait WaveGen {
    fn next_sample(&self, clock: f32) -> Sample;
}

pub struct Instrument {
    pub pitch: Pitch,
    pub oscilator: Box<Osc>,
}

impl WaveGen for Instrument {
    fn next_sample(&self, clock: f32) -> Sample {
        self.oscilator.next_sample(clock, self.pitch.freq(), 0.0)
    }
}
