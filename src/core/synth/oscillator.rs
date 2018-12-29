extern crate rand;

use super::{Sample, Seconds, ScaleRatio};
use core::music_theory::Hz;
use self::rand::{Rng, ThreadRng};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum Specs {
    Sine, Saw, Square, Pulse(ScaleRatio),
    Supersaw{ nvoices: u16, detune_amount: Hz }
}

#[derive(Copy, Clone)]
pub enum Modulation { PulseDuty, MixThickness }

pub trait Oscillator {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample;
    fn modulate(&mut self, modulation: Modulation, value: f64);
}
impl Oscillator {
    pub fn new(spec: Specs) -> Box<Oscillator> {
        match spec {
            Specs::Sine => Box::new(Sine),
            Specs::Square => Box::new(Square),
            Specs::Pulse(d) => Box::new(Pulse{duty_cycle: d}),
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
    fn modulate(&mut self, _modulation: Modulation, _value: f64) {}
}

pub struct Square;
impl Oscillator for Square {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq ) % 1.).round() * 2. - 1.
    }
    fn modulate(&mut self, _modulation: Modulation, _value: f64) {}
}

pub struct Pulse { duty_cycle: ScaleRatio }
impl Pulse {
    fn set_dutycycle(&mut self, value: f64) {
        let normalized = value.max(0.).min(1.);
        self.duty_cycle = normalized;
    }
}
impl Oscillator for Pulse {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        if ((clock + phase) * freq) % 1. < self.duty_cycle {1.} else {-1.}
    }
    fn modulate(&mut self, modulation: Modulation, value: f64) {
        match modulation {
            Modulation::PulseDuty => self.set_dutycycle(value),
            _ => ()
        }
    }
}

pub struct Saw;
impl Oscillator for Saw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq) % 1.)
    }
    fn modulate(&mut self, _modulation: Modulation, _value: f64) {}
}

pub struct StatefulSaw { pub detune: f64 }
impl Oscillator for StatefulSaw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let final_freq = freq + self.detune;
        Saw.next_sample(clock, final_freq, phase)
    }
    fn modulate(&mut self, _modulation: Modulation, _value: f64) {}
}

pub struct Mix { pub voices: Vec<Box<Oscillator>> }
impl Mix {
    fn supersaw(nvoices: u16, detune_amount: Hz) -> Mix {
        Mix { voices: Mix::create_voicees(nvoices, detune_amount) }
    }

    fn set_nvoices(&mut self, value: f64) {
        let bounded = value.max(0.).min(1.);
        let nvoices = (bounded * 32.).ceil() as u16;
        let detune = bounded * 9.;
        self.voices = Mix::create_voicees(nvoices, detune);
    }

    fn create_voicees(nvoices: u16, detune_amount: Hz) -> Vec<Box<Oscillator>> {
        let mut rng = rand::thread_rng();
        fn random_around_zero(rng: &mut ThreadRng, amount: Hz) -> Hz {
            rng.gen_range(-amount, amount)
        }

        let mut saws: Vec<Box<Oscillator>> = Vec::new();
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
    fn modulate(&mut self, modulation: Modulation, value: f64) {
        match modulation {
            Modulation::MixThickness => self.set_nvoices(value),
            _ => ()
        }
    }
}
