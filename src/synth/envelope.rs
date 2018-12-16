
use super::{Sample, Seconds, ScaleRatio};

#[derive(Clone, Copy)]
pub struct Adsr {
    pub attack: Seconds,
    pub decay: Seconds,
    pub sustain: ScaleRatio,
    pub release: Seconds,
}
impl Adsr {
    pub fn new(attack: Seconds, decay: Seconds, sustain: ScaleRatio, release: Seconds) -> Adsr {
        assert!(attack >= 0., "attack was: {}", attack);
        assert!(decay >= 0., "decay was: {}", decay);
        assert!(sustain >= 0. && sustain <= 1., "sustain was: {}", sustain);
        assert!(release >= 0., "release was: {}", release);
        Adsr { attack, decay, sustain, release }
    }

    pub fn apply(&self, elapsed: Seconds, elapsed_since_release: Seconds, sample: Sample) -> Sample {
        let scale_ratio =
            if elapsed_since_release > 0. {
                (1. - (elapsed_since_release / self.release)).max(0.)
            } else if elapsed < self.attack {
                (elapsed / self.attack)
            } else if elapsed < self.decay {
                1. - (elapsed - self.attack / self.decay)
            } else {
                self.sustain
            };
        sample * scale_ratio
    }
}