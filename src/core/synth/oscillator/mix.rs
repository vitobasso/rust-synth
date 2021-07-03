
use crate::core::synth::{Sample, Seconds, modulated::*};
use crate::core::music_theory::Hz;
use super::*;
use rand::{self, Rng};

pub struct Mix {
    voices: Vec<Voice>,
}

impl Mix {
    pub fn detuned(n_voices: usize, detune_amount: Hz, specs: &Specs) -> Mix {
        Mix { voices: create_voices(n_voices, detune_amount, specs) }
    }
}

fn create_voices(n_voices: usize, detune_amount: Hz, specs: &Specs) -> Vec<Voice> {
    let mut rng = rand::thread_rng();
    fn random_around_zero(rng: &mut rand::ThreadRng, amount: Hz) -> Hz {
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
}

impl Modulated<ModTarget> for Mix {
    fn mod_param(&mut self, _target: ModTarget) -> Option<&mut ModParam> { None }
}


struct Voice {
    tuning: Hz,
    oscillator: Box<dyn Oscillator>,
}

impl Voice {
    pub fn new(tuning: f64, specs: &Specs) -> Self {
        Self {
            tuning,
            oscillator: <dyn Oscillator>::new(specs),
        }
    }

    fn next_sample(&self, clock: Seconds, freq: Hz, phase: Seconds) -> Sample {
        let final_freq = freq + self.tuning;
        self.oscillator.next_sample(clock, final_freq, phase)
    }
}