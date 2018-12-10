use super::{Sample, pitch::Pitch, oscillator::Osc, filter::Filter};

pub struct Instrument {
    clock: Clock,
    pub oscillator: Box<Osc>,
    pub filter: Box<Filter>,
    pub pitch: Option<Pitch>,
    pub transpose: i8,
    mod_1: f64,
    mod_2: f64,
}

impl Instrument {

    pub fn new(sample_rate: f64, oscillator: Box<Osc>, filter: Box<Filter>) -> Instrument {
        let mut instrument = Instrument {
            oscillator,
            filter,
            clock: Clock::new(sample_rate),
            pitch: None,
            transpose: 0_i8,
            mod_1: 1.,
            mod_2: 0.,
        };
        instrument.update_filter();
        instrument
    }

    pub fn transposed_pitch(&self) -> Option<Pitch> {
        self.pitch.map(|p| p + self.transpose)
    }

    pub fn next_sample(&mut self) -> Option<Sample> {
        self.transposed_pitch().map(|p| {
            let clock = self.clock.tick();
            let raw = self.oscillator.next_sample(clock, p.freq(), 0.);
            self.filter.filter( raw)
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
        self.clock / self.sample_rate
    }
}