use super::{Sample, Seconds, Proportion, oscillator::{self, Oscillator},
            filter::{self, Filter}, envelope::Adsr, lfo::{self, LFO}, modulated::*};
use crate::core::music_theory::{Hz, pitch::Pitch};

#[derive(Clone, Copy)]
pub struct Specs {
    pub max_voices: u8,
    pub oscillator: oscillator::Specs,
    pub filter: filter::Specs,
    pub lfo: Option<lfo::Specs>,
    pub adsr: Adsr,
    pub volume: Proportion,
    pub modulation_x: ModTarget,
    pub modulation_y: ModTarget,
    pub modulation_lfo: ModSpecs,
}

#[derive(Copy, Clone)]
pub enum ModTarget {
    Noop, Volume,
    Filter(filter::ModTarget),
    Oscillator(oscillator::ModTarget),
}

#[derive(Copy, Clone)]
pub struct ModSpecs {
    pub target: ModTarget,
    pub amount: Proportion,
}

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    filter: Box<dyn Filter>,
    lfo: Option<LFO>,
    adsr: Adsr,
    volume: ModParam,
    voices: Voices,
    modulation_x: ModTarget,
    modulation_y: ModTarget,
    modulation_lfo: ModSpecs,
    clock: Clock,
}
impl Instrument {

    pub fn new(specs: Specs, sample_rate: Hz) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(specs.oscillator),
            filter: Filter::new(specs.filter, sample_rate),
            lfo: specs.lfo.map(LFO::new),
            adsr: specs.adsr,
            volume: ModParam::with_base(specs.volume, 0., 1.),
            modulation_x: specs.modulation_x,
            modulation_y: specs.modulation_y,
            modulation_lfo: specs.modulation_lfo,
            clock: Clock::new(sample_rate),
            voices: Voices::new(specs.max_voices, sample_rate, specs.adsr.release),
        }
    }

    pub fn hold(&mut self, pitch: Pitch) {
        self.voices.hold(pitch)
    }

    pub fn release(&mut self, pitch: Pitch) {
        self.voices.release(pitch)
    }

    pub fn release_all(&mut self) {
        self.voices.release_all()
    }

    pub fn next_sample(&mut self) -> Sample {
        self.run_next_lfo_modulation();
        let oscillator = &self.oscillator;
        let adsr = &self.adsr;
        self.voices.drop_finished_voices();
        let sample_mix: Sample = self.voices.voices.iter_mut()
            .map(|voice| Instrument::next_sample_for_voice(voice, oscillator, adsr))
            .sum();
        let sample_filtered = self.filter.filter(sample_mix);
        sample_filtered * self.volume.calculate()
    }

    fn next_sample_for_voice (voice: &mut Voice, oscillator: &Box<dyn Oscillator>, adsr: &Adsr) -> Sample {
        let clock = voice.clock.tick();
        let sample = oscillator.next_sample(clock, voice.pitch.freq(), 0.);
        adsr.apply(voice.clock(), voice.released_clock().unwrap_or(0.), sample)
    }

    pub fn set_xy_params(&mut self, x: f64, y: f64) {
        let x_target = self.modulation_x;
        let y_target = self.modulation_y;
        if let Some(param) = self.mod_param(x_target){
            param.set_base(x);
        }
        if let Some(param) = self.mod_param(y_target){
            param.set_base(y);
        }
    }

    pub fn set_oscillator(&mut self, specs: oscillator::Specs) {
        self.oscillator = Oscillator::new(specs)
    }

    fn run_next_lfo_modulation(&mut self) {
        let maybe_lfo_sample = {
            let clock_ref = &mut self.clock;
            self.lfo.as_ref().map(|lfo| {
                let clock = clock_ref.tick();
                lfo.next(clock)
            })
        };
        let specs = self.modulation_lfo;
        if let Some(lfo_sample) = maybe_lfo_sample {
            let normalized = (lfo_sample + 1.) / 2.;
            if let Some(param) = self.mod_param(specs.target) {
                param.set_signal(normalized * specs.amount);
            }
        }
    }

}

impl Modulated<ModTarget> for Instrument {
    fn mod_param(&mut self, target: ModTarget) -> Option<&mut ModParam> {
        match target {
            ModTarget::Noop => None,
            ModTarget::Volume => Some(&mut self.volume),
            ModTarget::Filter(m) => self.filter.mod_param(m),
            ModTarget::Oscillator(m) => self.oscillator.mod_param(m),
        }
    }
}

struct Voices {
    max_voices: u8,
    voices: Vec<Voice>,
    sample_rate: Hz,
    release: Seconds,
}
impl Voices {
    fn new(max_voices: u8, sample_rate: Hz, release: Seconds,) -> Voices {
        Voices{ max_voices, voices: vec![], sample_rate, release }
    }

    fn hold(&mut self, pitch: Pitch) {
        if self.has_free_voice() {
            self.voices.push(Voice::new(self.sample_rate, pitch))
        }
    }

    fn release(&mut self, pitch: Pitch) {
        if let Some(voice) = self.find_holding_voice(pitch) {
            voice.release();
        }
    }

    fn release_all(&mut self) {
        self.voices.iter_mut().for_each(|v| v.release());
    }

    fn find_holding_voice(&mut self, pitch: Pitch) -> Option<&mut Voice> {
        self.voices.iter_mut()
            .find(|v| v.pitch == pitch && v.is_holding())
    }

    fn drop_finished_voices(&mut self) {
        let release = self.release;
        self.voices.retain(|voice| !voice.is_finished(release))
    }

    fn has_free_voice(&self) -> bool {
        self.voices.len() < self.max_voices as usize
    }

}

struct Voice {
    pitch: Pitch,
    released_at: Option<Seconds>,
    clock: Clock,
}
impl Voice {

    fn new(sample_rate: Hz, pitch: Pitch) -> Voice {
        Voice {
            pitch,
            released_at: None,
            clock: Clock::new(sample_rate)
        }
    }

    fn clock(&self) -> Seconds {
        self.clock.get()
    }

    fn release(&mut self) {
        if self.is_holding() {
            self.released_at = Some(self.clock.get())
        }
    }

    fn released_clock(&self) -> Option<Seconds> {
        self.released_at.as_ref().map(|begin| self.clock() - begin)
    }

    fn is_holding(&self) -> bool {
        self.released_at.is_none()
    }

    fn is_finished(&self, decay: Seconds) -> bool {
        let now = self.clock.get();
        self.released_at.map(|released| now - released > decay).unwrap_or(false)
    }

}

struct Clock {
    sample_rate: Hz,
    clock: f64,
}
impl Clock {

    fn new(sample_rate: Hz) -> Clock {
        Clock{ sample_rate, clock: 0. }
    }

    fn tick(&mut self) -> Seconds {
        self.clock += 1.0;
        self.get()
    }

    fn get(&self) -> Seconds {
        self.clock / self.sample_rate
    }

}

impl Default for Specs {
    fn default() -> Self {
        Specs {
            max_voices: 8,
            oscillator: oscillator::Specs::default(),
            filter: filter::Specs::default(),
            lfo: None,
            adsr: Adsr::default(),
            volume: 1.,
            modulation_x: ModTarget::default(),
            modulation_y: ModTarget::default(),
            modulation_lfo: ModSpecs::default(),
        }
    }
}

impl Default for ModTarget {
    fn default() -> Self {
        ModTarget::Noop
    }
}

impl Default for ModSpecs {
    fn default() -> Self {
        ModSpecs { target: ModTarget::Noop, amount: 1. }
    }
}

