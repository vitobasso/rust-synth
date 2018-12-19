use super::{Sample, Seconds, Hz, ScaleRatio, pitch::Pitch,
            oscillator::{self, Oscillator}, filter::{self, Filter}, envelope::Adsr, };

#[derive(Clone, Copy)]
pub struct Specs {
    pub oscillator: oscillator::Specs,
    pub filter: filter::Specs,
    pub adsr: Adsr,
    pub amplify: ScaleRatio,
}

pub struct Instrument {
    sample_rate: Hz,
    pub oscillator: Box<Oscillator>,
    pub filter: Box<Filter>,
    pub adsr: Adsr,
    voice: Option<Voice>,
    amplify: ScaleRatio,
}

impl Instrument {

    pub fn new(specs: Specs, sample_rate: Hz) -> Instrument {
        let oscillator = Oscillator::new(specs.oscillator);
        let filter = Filter::new(specs.filter, sample_rate);
        let adsr = specs.adsr;
        let amplify = specs.amplify;
        Instrument {
            sample_rate, oscillator, filter, adsr, amplify, voice: None
        }
    }

    pub fn hold(&mut self, pitch: Pitch) {
        if !self.is_holding(pitch) {
            self.voice = Some(Voice::new(self.sample_rate, pitch));
        }
    }

    pub fn release(&mut self, pitch: Pitch) {
        if self.is_holding(pitch) {
            self.release_any()
        }
    }

    pub fn release_any(&mut self) {
        self.voice.as_mut().map(|v| v.release());
    }

    pub fn is_holding(&self, pitch: Pitch) -> bool {
        self.voice.as_ref()
            .map(|v| v.pitch == pitch && v.is_holding())
            .unwrap_or(false)
    }

    pub fn next_sample(&mut self) -> Option<Sample> {
        let oscillator = &self.oscillator;
        let filter = &mut self.filter;
        let adsr = &self.adsr;
        let amplify = &self.amplify;
        self.voice.as_mut().map(|v| {
            let clock = v.clock.tick();
            let sample_raw = oscillator.next_sample(clock, v.pitch.freq(), 0.);
            let sample_filtered = filter.filter( sample_raw);
            let sample_adsr = adsr.apply(v.clock(), v.released_clock().unwrap_or(0.), sample_filtered);
            sample_adsr * amplify
        })
    }

    pub fn set_params(&mut self, x: f64, y: f64) {
        let params = filter::Params{cutoff: y, q_factor: x};
        self.filter.set_params(params)
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
        self.released_at = Some(self.clock.get())
    }
    fn released_clock(&self) -> Option<Seconds> {
        self.released_at.as_ref().map(|begin| self.clock() - begin)
    }
    fn is_holding(&self) -> bool {
        self.released_at.is_none()
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