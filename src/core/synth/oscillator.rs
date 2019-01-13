use rand;

use super::{Sample, Seconds, Proportion, modulated::*};
use crate::core::music_theory::Hz;
use self::rand::{Rng, ThreadRng};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum Specs {
    Sine, Saw, Square, Pulse(Proportion),
    Supersaw{ nvoices: u16, detune_amount: Hz }
}

#[derive(Copy, Clone)]
pub enum ModTarget { PulseDuty, MixThickness }

pub trait Oscillator: Modulated<ModTarget> {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample;
}
impl dyn Oscillator {
    pub fn new(spec: Specs) -> Box<dyn Oscillator> {
        match spec {
            Specs::Sine => Box::new(Sine),
            Specs::Square => Box::new(Square),
            Specs::Pulse(d) => Box::new(Pulse::new(d)),
            Specs::Saw => Box::new(Saw),
            Specs::Supersaw{nvoices: v, detune_amount: d} =>
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
impl Modulated<ModTarget> for Sine {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct Square;
impl Oscillator for Square {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq ) % 1.).round() * 2. - 1.
    }
}
impl Modulated<ModTarget> for Square {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct Pulse {
    duty_cycle: ModParam,
}
impl Pulse {
    fn new(duty_cycle: Proportion) -> Pulse {
        Pulse { duty_cycle: ModParam::with_base(duty_cycle, 0., 1.) }
    }
}
impl Oscillator for Pulse {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let duty_cycle = self.duty_cycle.calculate();
        if ((clock + phase) * freq) % 1. < duty_cycle {1.} else {-1.}
    }
}
impl Modulated<ModTarget> for Pulse {
    fn mod_param(&mut self, target: ModTarget) -> Option<&mut ModParam> {
        match target {
            ModTarget::PulseDuty => Some(&mut self.duty_cycle),
            _ => None
        }
    }
}

pub struct Saw;
impl Oscillator for Saw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq) % 1.)
    }
}
impl Modulated<ModTarget> for Saw {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct StatefulSaw { pub detune: f64 }
impl Oscillator for StatefulSaw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let final_freq = freq + self.detune;
        Saw.next_sample(clock, final_freq, phase)
    }
}
impl Modulated<ModTarget> for StatefulSaw {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct Mix {
    voices: Vec<Box<dyn Oscillator>>,
}
impl Mix {
    fn supersaw(nvoices: u16, detune_amount: Hz) -> Mix {
        Mix {
            voices: Mix::create_voices(nvoices, detune_amount),
        }
    }

    fn create_voices(nvoices: u16, detune_amount: Hz) -> Vec<Box<dyn Oscillator>> {
        let mut rng = rand::thread_rng();
        fn random_around_zero(rng: &mut ThreadRng, amount: Hz) -> Hz {
            rng.gen_range(-amount, amount)
        }

        let mut saws: Vec<Box<dyn Oscillator>> = Vec::new();
        for _ in 0..nvoices {
            let saw = StatefulSaw { detune: random_around_zero(&mut rng, detune_amount) };
            saws.push(Box::new(saw))
        }
        saws
    }

}
impl Oscillator for Mix {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        self.voices.iter()
            .map(|o| o.next_sample(clock, freq, phase))
            .sum()
    }
}
impl Modulated<ModTarget> for Mix {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}