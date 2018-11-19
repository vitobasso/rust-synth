
pub mod oscillator;
pub mod filter;
use self::oscillator::Osc;
use self::filter::Filter;
use pitches::Pitch;

type Sample = f64;

pub trait WaveGen {
    fn set_mod_1(&mut self, value: f64) -> ();
    fn set_mod_2(&mut self, value: f64) -> ();
    fn next_sample(&mut self, clock: f64) -> Sample;
}

pub struct Instrument {
    pub sample_rate: f64,
    pub oscillator: Box<Osc>,
    pub filter: Box<Filter>,
    pub pitch: Pitch,
    pub mod_1: f64,
    pub mod_2: f64,
}
impl Instrument {
    pub fn new(sample_rate: f64, oscillator: Box<Osc>, filter: Box<Filter>) -> Instrument {
        let mut instrument = Instrument {
            sample_rate,
            pitch: Pitch::default(),
            oscillator,
            filter,
            mod_1: 1.,
            mod_2: 0.,
        };
        instrument.update_filter();
        instrument
    }
    fn update_filter(&mut self) -> () {
        self.filter.set_params(self.mod_1, self.mod_2, self.sample_rate)
    }
}

impl WaveGen for Instrument {
    fn set_mod_1(&mut self, value: f64) -> () {
        self.mod_1 = value;
        self.update_filter();
    }
    fn set_mod_2(&mut self, value: f64) -> () {
        self.mod_2 = value;
        self.update_filter();
    }
    fn next_sample(&mut self, clock: f64) -> Sample {
        let raw = self.oscillator.next_sample(clock, self.pitch.freq(), 0.);
        self.filter.filter( raw)
    }
}
