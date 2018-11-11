use std::f32::consts::PI;
use pitches::Pitch;

pub trait Osc {
    fn next_sample(&self, clock: f32, freq: f32) -> f32;
}

pub struct Sine;
impl Osc for Sine {
    fn next_sample(&self, clock: f32, freq: f32) -> f32 {
        (clock * freq * 2.0 * PI).sin()
    }
}

pub struct Saw;
impl Osc for Saw {
    fn next_sample(&self, clock: f32, freq: f32) -> f32 {
        ((clock * freq) % 1.0)
    }
}

pub struct Switch { pub is_saw: bool }
impl Osc for Switch {
    fn next_sample(&self, clock: f32, freq: f32) -> f32 {
        if self.is_saw {
            Saw.next_sample(clock, freq)
        } else {
            Sine.next_sample(clock, freq)
        }
    }
}


pub trait WaveGen {
    fn next_sample(&self, clock: f32) -> f32;
}

pub struct Instrument<T: Osc> {
    pub pitch: Pitch,
    pub osc: T,
}

impl <T: Osc> WaveGen for Instrument<T> {
    fn next_sample(&self, clock: f32) -> f32 {
        self.osc.next_sample(clock, self.pitch.freq())
    }
}
