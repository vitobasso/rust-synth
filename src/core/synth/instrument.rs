use super::{Sample, Seconds, Proportion, Velocity, oscillator::{self, Oscillator},
            filter::{self, Filter}, adsr::Adsr, lfo::{self, LFO}, modulated::*};
use crate::core::music_theory::{Hz, pitch::Pitch};

///
/// Connects modules of the synthesizer together to produce a stream of sound samples.
///

#[derive(Clone, PartialEq, Debug)]
pub struct Specs {
    pub max_voices: u8,
    pub oscillator: oscillator::Specs,
    pub filter: filter::Specs,
    pub lfo: Option<lfo::Specs>,
    pub adsr: Adsr,
    pub volume: Proportion,
    pub modulation_x: ModTarget,
    pub modulation_y: ModTarget,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModTarget {
    Noop, Volume,
    Filter(filter::ModTarget),
    Oscillator(oscillator::ModTarget),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ModSpecs {
    pub target: ModTarget,
    pub amount: Proportion,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub oscillator: oscillator::View,
    pub filter: filter::View,
    pub lfo: Option<lfo::View>,
    pub adsr: Adsr,
    pub volume: Proportion,
}

#[derive(Clone)]
pub struct State {
    voices: Voices,
    clock: Clock,
}

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    filter: Box<dyn Filter>,
    lfo: Option<LFO>,
    adsr: Adsr,
    volume: ModParam,
    modulation_x: ModTarget,
    modulation_y: ModTarget,
    voices: Voices,
    clock: Clock,
}

impl Instrument {

    pub fn new(specs: Specs, sample_rate: Hz) -> Instrument {
        Instrument {
            oscillator: <dyn Oscillator>::new(&specs.oscillator),
            filter: <dyn Filter>::new(specs.filter, sample_rate),
            lfo: specs.lfo.map(LFO::new),
            adsr: specs.adsr,
            volume: ModParam::with_base(specs.volume, 0., 1.),
            modulation_x: specs.modulation_x,
            modulation_y: specs.modulation_y,
            clock: Clock::new(sample_rate),
            voices: Voices::new(specs.max_voices, sample_rate, specs.adsr.release),
        }
    }

    pub fn hold(&mut self, pitch: Pitch, velocity: Velocity) {
        self.voices.hold(pitch, velocity)
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

    fn next_sample_for_voice(voice: &mut Voice, oscillator: &Box<dyn Oscillator>, adsr: &Adsr) -> Sample {
        let clock = voice.clock.tick();
        let sample = oscillator.next_sample(clock, voice.pitch.freq(), 0.) * voice.velocity;
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

    fn run_next_lfo_modulation(&mut self) {
        if let Some(lfo) = &self.lfo {
            let clock = self.clock.tick();
            let sample = lfo.next(clock);
            let target = lfo.target.clone();
            if let Some(param) = self.mod_param(target) {
                param.set_signal(sample);
            }
        }
    }

    pub fn get_state(&self) -> State{
        State {
            voices: self.voices.clone(),
            clock: self.clock.clone(),
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.clock = state.clock;
        self.voices = state.voices;
    }

    pub fn view(&self) -> View {
        View {
            filter: self.filter.view(),
            oscillator: self.oscillator.view(),
            lfo: self.lfo.as_ref().map(|l| l.view()),
            adsr: self.adsr.clone(),
            volume: self.volume.normalized(),
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

#[derive(Clone)]
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

    fn hold(&mut self, pitch: Pitch, velocity: Velocity) {
        if self.has_free_voice() {
            self.voices.push(Voice::new(self.sample_rate, pitch, velocity))
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

#[derive(Clone)]
struct Voice {
    pitch: Pitch,
    velocity: Velocity,
    released_at: Option<Seconds>,
    clock: Clock, //TODO share clock from Instrument?
}
impl Voice {

    fn new(sample_rate: Hz, pitch: Pitch, velocity: Velocity) -> Voice {
        Voice {
            pitch, velocity,
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

#[derive(Clone)]
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
