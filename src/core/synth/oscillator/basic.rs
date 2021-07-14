
use std::f64::consts::PI;
use super::*;

pub struct Sine;
impl Oscillator for Sine {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        ((clock + phase) * freq * 2. * PI).sin()
    }

    fn view(&self) -> View {
        View::Sine
    }

    fn set_specs(&mut self, _: Specs) {}
}
impl Modulated<ModTarget> for Sine {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct Square;
impl Oscillator for Square {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        (((clock + phase) * freq ) % 1.).round() * 2. - 1.
    }

    fn view(&self) -> View {
        View::Square
    }

    fn set_specs(&mut self, _: Specs) {}
}
impl Modulated<ModTarget> for Square {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}

pub struct Saw;
impl Oscillator for Saw {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        ((clock + phase) * freq) % 1.
    }

    fn view(&self) -> View {
        View::Saw
    }

    fn set_specs(&mut self, _: Specs) {}
}
impl Modulated<ModTarget> for Saw {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}
