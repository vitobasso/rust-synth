
use super::*;

pub struct Pulse {
    duty_cycle: ModParam,
}
impl Pulse {

    pub fn new(duty_cycle: Proportion) -> Self {
        Self::restored(duty_cycle, 0.)
    }

    pub fn restored(base: Proportion, modulation: f64) -> Self {
        let param = ModParam {
            base,
            mod_signal: modulation,
            min: 0.,
            range: 1.,
        };
        Self { duty_cycle: param }
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

    fn state(&self) -> State {
        State::Pulse(self.duty_cycle.mod_signal)
    }
}
impl Modulated<ModTarget> for Pulse {
    fn mod_param(&mut self, target: ModTarget) -> Option<&mut ModParam> {
        match target {
            ModTarget::PulseDuty => Some(&mut self.duty_cycle),
        }
    }
}
