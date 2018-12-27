extern crate rand;

use super::{Sample, Seconds, ScaleRatio};
use core::music_theory::Hz;
use self::rand::{Rng, ThreadRng};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum Specs {
    Sine, Saw, Square,
    Pulse(ScaleRatio),
    Supersaw{n_voices: u16, detune_amount: Hz}
}

pub trait Oscillator {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample;
}
impl Oscillator {
    pub fn new(spec: Specs) -> Box<Oscillator> {
        match spec {
            Specs::Sine => Box::new(Sine),
            Specs::Square => Box::new(Square),
            Specs::Pulse(d) => Box::new(Pulse{duty_cycle: d}),
            Specs::Saw => Box::new(Saw),
            Specs::Supersaw{n_voices: v, detune_amount: d} =>
                Box::new(Mix::supersaw(v, d)),
        }
    }
}

pub struct Sine;
impl Oscillator for Sine {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        ((clock + phase) * freq * 2. * PI).sin()
    }
}

pub struct Square;
impl Oscillator for Square {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq ) % 1.).round() * 2. - 1.
    }
}

pub struct Pulse { duty_cycle: ScaleRatio }
impl Oscillator for Pulse {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        if ((clock + phase) * freq) % 1. < self.duty_cycle {1.} else {-1.}
    }
}

pub struct Saw;
impl Oscillator for Saw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq) % 1.)
    }
}

pub struct StatefulSaw { pub detune: f64 }
impl Oscillator for StatefulSaw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let final_freq = freq + self.detune;
        Saw.next_sample(clock, final_freq, phase)
    }
}

pub struct Mix { pub voices: Vec<Box<Oscillator>> }
impl Oscillator for Mix {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        self.voices.iter()
            .map(|o| o.next_sample(clock, freq, phase))
            .sum()
    }
}
impl Mix {
    fn supersaw(n_voices: u16, detune_amount: Hz) -> Mix {
        let mut rng = rand::thread_rng();
        fn random_around_zero(rng: &mut ThreadRng, amount: Hz) -> Hz {
            rng.gen_range(-amount, amount)
        }

        let mut saws: Vec<Box<Oscillator>> = Vec::new();
        for _ in 0..n_voices {
            let saw = StatefulSaw { detune: random_around_zero(&mut rng, detune_amount) };
            saws.push(Box::new(saw))
        }
        Mix { voices: saws }
    }
}
