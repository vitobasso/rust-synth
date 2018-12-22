use super::{Sample, Seconds, Hz, ScaleRatio, pitch::Pitch,
            oscillator::{self, Oscillator}, filter::{self, Filter}, envelope::Adsr, };

#[derive(Clone, Copy)]
pub struct Specs {
    pub max_voices: u8,
    pub oscillator: oscillator::Specs,
    pub filter: filter::Specs,
    pub adsr: Adsr,
    pub amplify: ScaleRatio,
}

pub struct Instrument {
    oscillator: Box<Oscillator>,
    filter: Box<Filter>,
    adsr: Adsr,
    amplify: ScaleRatio,
    voices: Voices,
}
impl Instrument {

    pub fn new(specs: Specs, sample_rate: Hz) -> Instrument {
        let oscillator = Oscillator::new(specs.oscillator);
        let filter = Filter::new(specs.filter, sample_rate);
        let adsr = specs.adsr;
        let amplify = specs.amplify;
        let voices = Voices::new(specs.max_voices, sample_rate, adsr.release);
        Instrument { oscillator, filter, adsr, amplify, voices }
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
        let oscillator = &self.oscillator;
        let filter = &mut self.filter;
        let adsr = &self.adsr;
        self.voices.drop_finished_voices();
        self.voices.voices.iter_mut()
            .map(|voice| Instrument::next_sample_for_voice(voice, oscillator, filter, adsr))
            .fold(0., |a, b| a + b) * self.amplify
    }

    fn next_sample_for_voice (voice: &mut Voice, oscillator: &Box<Oscillator>, filter: &mut Box<Filter>, adsr: &Adsr) -> Sample {
        let clock = voice.clock.tick();
        let sample_raw = oscillator.next_sample(clock, voice.pitch.freq(), 0.);
        let sample_filtered = filter.filter( sample_raw);
        adsr.apply(voice.clock(), voice.released_clock().unwrap_or(0.), sample_filtered)
    }

    pub fn set_params(&mut self, x: f64, y: f64) {
        let params = filter::Params{cutoff: y, q_factor: x};
        self.filter.set_params(params)
    }

    pub fn set_oscillator(&mut self, specs: oscillator::Specs) {
        self.oscillator = Oscillator::new(specs)
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
        if self.find_holding_voice(pitch).is_none() && self.has_free_voice() {
            self.voices.push(Voice::new(self.sample_rate, pitch))
        }
    }

    fn release(&mut self, pitch: Pitch) {
        self.find_holding_voice(pitch)
            .map(|v| v.release());
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
        self.clock = self.clock + 1.0;
        self.get()
    }

    fn get(&self) -> Seconds {
        self.clock / self.sample_rate
    }

}