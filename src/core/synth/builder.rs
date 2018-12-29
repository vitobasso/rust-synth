use super::{Seconds, ScaleRatio, instrument::{Specs, Modulation}, oscillator, filter, envelope::Adsr};

pub struct Builder {
    max_voices: u8,
    oscillator: oscillator::Specs,
    filter: filter::Specs,
    adsr: Adsr,
    volume: ScaleRatio,
    modulation_x: Modulation,
    modulation_y: Modulation,
}
impl Builder {

    pub fn osc(oscillator: oscillator::Specs) -> Builder {
        Builder {
            max_voices: 8,
            oscillator: oscillator,
            filter: filter::Specs::LPF,
            adsr: Adsr::new(0., 0.05, 0.8, 0.2),
            volume: 1.,
            modulation_x: Modulation::Filter(filter::Modulation::Cutoff),
            modulation_y: Modulation::Filter(filter::Modulation::QFactor),
        }
    }

    pub fn build(self) -> Specs {
        Specs {
            max_voices: self.max_voices,
            oscillator: self.oscillator,
            filter: self.filter,
            adsr: self.adsr,
            volume: self.volume,
            modulation_x: self.modulation_x,
            modulation_y: self.modulation_y,
        }
    }

    pub fn filter(mut self, value: filter::Specs) -> Self {
        self.filter = value;
        self
    }
    pub fn attack(mut self, value: Seconds) -> Self {
        self.adsr.attack = value;
        self
    }
    pub fn decay(mut self, value: Seconds) -> Self {
        self.adsr.decay = value;
        self
    }
    pub fn sustain(mut self, value: ScaleRatio) -> Self {
        self.adsr.sustain = value;
        self
    }
    pub fn release(mut self, value: Seconds) -> Self {
        self.adsr.release = value;
        self
    }
    pub fn adsr(mut self, a: Seconds, d: Seconds, s: ScaleRatio, r: Seconds) -> Self {
        self.adsr = Adsr::new(a, d, s, r);
        self
    }
    pub fn volume(mut self, value: ScaleRatio) -> Self {
        self.volume = value;
        self
    }
    pub fn mod_x(mut self, value: Modulation) -> Self {
        self.modulation_x = value;
        self
    }
    pub fn mod_y(mut self, value: Modulation) -> Self {
        self.modulation_y = value;
        self
    }
}