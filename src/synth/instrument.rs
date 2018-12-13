use super::{Sample, pitch::Pitch, oscillator::Osc, filter::Filter, modulation::Adsr};

pub struct Instrument {
    sample_rate: f64,
    pub oscillator: Box<Osc>,
    pub filter: Box<Filter>,
    pub adsr: Adsr,
    pub transpose: i8,
    voice: Option<Voice>,
    mod_1: f64,
    mod_2: f64,
}

impl Instrument {

    pub fn new(sample_rate: f64, oscillator: Box<Osc>, filter: Box<Filter>, adsr: Adsr) -> Instrument {
        let mut instrument = Instrument {
            sample_rate, oscillator, filter, adsr,
            voice: None,
            transpose: 0_i8,
            mod_1: 1., mod_2: 0.,
        };
        instrument.update_filter();
        instrument
    }

    pub fn hold(&mut self, pitch: Pitch) {
        self.voice = Some(Voice::new(self.sample_rate, pitch));
    }

    pub fn release(&mut self) {
        self.voice.as_mut().map(|v| v.release());
    }

    pub fn is_holding(&self, pitch: Pitch) -> bool {
        self.voice.as_ref()
            .map(|v| v.pitch == pitch && v.is_holding())
            .unwrap_or(false)
    }

    pub fn next_sample(&mut self) -> Option<Sample> {
        let transpose = self.transpose.clone();
        let oscillator = &self.oscillator;
        let filter = &mut self.filter;
        let adsr = &self.adsr;
        self.voice.as_mut().map(|v| {
            let clock = v.clock.tick();
            let pitch = v.transposed_pitch(transpose);
            let raw = oscillator.next_sample(clock, pitch.freq(), 0.);
            let filtered = filter.filter( raw);
            adsr.modulate(v.clock(), v.released_clock().unwrap_or(0.), filtered)
        })
    }

    pub fn set_mod_1(&mut self, value: f64) -> () {
        self.mod_1 = value;
        self.update_filter();
    }

    pub fn set_mod_2(&mut self, value: f64) -> () {
        self.mod_2 = value;
        self.update_filter();
    }

    fn update_filter(&mut self) -> () {
        self.filter.set_params(self.mod_1, self.mod_2)
    }
}

struct Voice {
    pitch: Pitch,
    released_at: Option<f64>,
    clock: Clock,
}
impl Voice {
    fn new(sample_rate: f64, pitch: Pitch) -> Voice {
        Voice {
            pitch,
            released_at: None,
            clock: Clock::new(sample_rate)
        }
    }
    fn transposed_pitch(&self, transpose: i8) -> Pitch {
        self.pitch + transpose
    }
    fn clock(&self) -> f64 {
        self.clock.get()
    }
    fn release(&mut self) {
        self.released_at = Some(self.clock.get())
    }
    fn released_clock(&self) -> Option<f64> {
        self.released_at.as_ref().map(|begin| self.clock() - begin)
    }
    fn is_holding(&self) -> bool {
        self.released_at.is_none()
    }
}

struct Clock {
    sample_rate: f64,
    clock: f64,
}
impl Clock {
    fn new(sample_rate: f64) -> Clock {
        Clock{ sample_rate, clock: 0. }
    }
    fn tick(&mut self) -> f64 {
        self.clock = self.clock + 1.0;
        self.get()
    }
    fn get(&self) -> f64 {
        self.clock / self.sample_rate
    }
}