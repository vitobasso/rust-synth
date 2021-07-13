use super::{Seconds, Proportion, instrument::{self, ModTarget}, oscillator, filter, adsr::Adsr, lfo};

pub struct Builder {
    max_voices: u8,
    oscillator: oscillator::Specs,
    filter: filter::Specs,
    lfo: Option<lfo::Specs>,
    adsr: Adsr,
    volume: Proportion,
    modulation_x: ModTarget,
    modulation_y: ModTarget,
}
impl Builder {

    pub fn osc(oscillator: oscillator::Specs) -> Builder {
        Builder {
            oscillator,
            max_voices: 8,
            filter: filter::Specs::default(),
            lfo: None,
            adsr: Adsr::new(0., 0.05, 0.8, 0.2),
            volume: 0.2,
            modulation_x: ModTarget::Filter(filter::ModTarget::Cutoff),
            modulation_y: ModTarget::Filter(filter::ModTarget::QFactor),
        }
    }

    pub fn build(self) -> instrument::Specs {
        instrument::Specs {
            max_voices: self.max_voices,
            oscillator: self.oscillator,
            filter: self.filter,
            lfo: self.lfo,
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
    pub fn sustain(mut self, value: Proportion) -> Self {
        self.adsr.sustain = value;
        self
    }
    pub fn release(mut self, value: Seconds) -> Self {
        self.adsr.release = value;
        self
    }
    pub fn lfo(mut self, value: lfo::Specs) -> Self {
        self.lfo = Some(value);
        self
    }
    pub fn adsr(mut self, a: Seconds, d: Seconds, s: Proportion, r: Seconds) -> Self {
        self.adsr = Adsr::new(a, d, s, r);
        self
    }
    pub fn volume(mut self, value: Proportion) -> Self {
        self.volume = value;
        self
    }
    pub fn mod_x(mut self, target: ModTarget) -> Self {
        self.modulation_x = target;
        self
    }
    pub fn mod_y(mut self, target: ModTarget) -> Self {
        self.modulation_y = target;
        self
    }
}