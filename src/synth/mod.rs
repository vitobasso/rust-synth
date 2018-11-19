
pub mod oscillator;
pub mod filter;
use self::oscillator::Osc;
use self::filter::Filter;
use pitches::Pitch;

type Sample = f64;

pub trait WaveGen {
    fn next_sample(&mut self, clock: f64, sample_rate: f64) -> Sample;
}

pub struct Instrument {
    pub pitch: Pitch,
    pub oscillator: Box<Osc>,
    pub filter: Box<Filter>,
    pub mod_param_1: f64,
    pub mod_param_2: f64,
}

impl WaveGen for Instrument {
    fn next_sample(&mut self, clock: f64, sample_rate: f64) -> Sample {
        let raw = self.oscillator.next_sample(clock, self.pitch.freq(), 0.);
        self.filter.filter(self.mod_param_1, self.mod_param_2, raw, sample_rate)
    }
}
