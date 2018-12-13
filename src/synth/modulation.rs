
use super::Sample;

/// The time params `attack`, `decay` and `release` are in seconds.
/// `sustain` is a level between 0 and 1
pub struct Adsr {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}
impl Adsr {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Adsr {
        assert!(attack >= 0., "attack was: {}", attack);
        assert!(decay >= 0., "decay was: {}", decay);
        assert!(sustain >= 0. && sustain <= 1., "sustain was: {}", sustain);
        assert!(release >= 0., "release was: {}", release);
        Adsr { attack, decay, sustain, release }
    }

    /// `clock` measures the time elapsed from:
    ///     - when the note was triggered, if `is_holding` is true.
    ///     - when note was released otherwise.
    pub fn modulate(&self, clock: f64, release_clock: f64, sample: Sample) -> Sample {
        let scale =
            if release_clock > 0. {
                (1. - (release_clock / self.release)).max(0.)
            } else if clock < self.attack {
                (clock / self.attack)
            } else if clock < self.decay {
                1. - (clock - self.attack / self.decay)
            } else {
                self.sustain
            };
        sample * scale
    }
}