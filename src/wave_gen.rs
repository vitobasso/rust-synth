use std::f32::consts::PI;

pub trait WaveGen {
    fn next_sample(&self, clock: f32) -> f32;
}

pub struct Sine;
impl WaveGen for Sine {
    fn next_sample(&self, clock: f32) -> f32 {
        (clock * 440.0 * 2.0 * PI).sin()
    }
}

pub struct Saw;
impl WaveGen for Saw {
    fn next_sample(&self, clock: f32) -> f32 {
        ((clock * 440.0) % 1.0)
    }
}

pub struct Switch { pub is_saw: bool }
impl WaveGen for Switch {
    fn next_sample(&self, clock: f32) -> f32 {
        if self.is_saw {
            Saw.next_sample(clock)
        } else {
            Sine.next_sample(clock)
        }
    }
}
