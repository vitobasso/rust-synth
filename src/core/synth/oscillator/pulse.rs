
use super::*;

pub struct Pulse {
    duty_cycle: ModParam,
}
impl Pulse {
    pub fn new(duty_cycle: Proportion) -> Pulse {
        Pulse { duty_cycle: ModParam::with_base(duty_cycle, 0., 1.) }
    }
}
impl Oscillator for Pulse {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let duty_cycle = self.duty_cycle.calculate();
        if ((clock + phase) * freq) % 1. < duty_cycle {1.} else {-1.}
    }

    fn view(&self) -> View {
        View::Pulse(self.duty_cycle.normalized())
    }
}
impl Modulated<ModTarget> for Pulse {
    fn mod_param(&mut self, target: ModTarget) -> Option<&mut ModParam> {
        match target {
            ModTarget::PulseDuty => Some(&mut self.duty_cycle),
        }
    }
}
