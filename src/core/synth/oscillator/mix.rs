
use crate::core::synth::{Sample, Seconds, modulated::*};
use crate::core::music_theory::Hz;
use super::*;
use rand::{self, Rng, StdRng, SeedableRng};

pub struct Mix {
    voices: Vec<Voice>,
}

impl Mix {
    pub fn detuned(n_voices: usize, detune_amount: Hz, specs: Basic, random_seed: u64) -> Mix {
        Mix { voices: create_voices(n_voices, detune_amount, specs, random_seed) }
    }
}

fn create_voices(n_voices: usize, detune_amount: Hz, specs: Basic, random_seed: u64) -> Vec<Voice> {
    let mut rng = StdRng::seed_from_u64(random_seed);
    fn random_around_zero(rng: &mut StdRng, amount: Hz) -> Hz {
        rng.gen_range(-amount, amount)
    }

    vec![0; n_voices].iter()
        .map(|_| Voice::new(random_around_zero(&mut rng, detune_amount), specs))
        .collect()
}

impl Oscillator for Mix {
    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        self.voices.iter()
            .map(|v| v.next_sample(clock, freq, phase))
            .sum()
    }

    fn view(&self) -> View {
        View::Mix {
            voices: self.voices.iter().map(|v| v.view()).collect()
        }
    }

    fn state(&self) -> State {
        State::Empty
    }
}

impl Modulated<ModTarget> for Mix {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}


struct Voice {
    tuning: Hz,
    oscillator: Box<dyn Oscillator>,
}

impl Voice {
    pub fn new(tuning: f64, specs: Basic) -> Self {
        Self {
            tuning,
            oscillator: <dyn Oscillator>::new(&Specs::Basic(specs)),
        }
    }

    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let final_freq = freq + self.tuning;
        self.oscillator.next_sample(clock, final_freq, phase)
    }

    fn view(&self) -> MixVoiceView {
        MixVoiceView {
            tuning: self.tuning,
            oscillator: Box::new(self.oscillator.view())
        }
    }
}