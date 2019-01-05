use super::{Sample, Seconds, Proportion};

#[derive(Clone, Copy)]
pub struct Adsr {
    pub attack: Seconds,
    pub decay: Seconds,
    pub sustain: Proportion,
    pub release: Seconds,
}
impl Adsr {
    pub fn new(attack: Seconds, decay: Seconds, sustain: Proportion, release: Seconds) -> Adsr {
        assert!(attack >= 0., "attack was: {}", attack);
        assert!(decay >= 0., "decay was: {}", decay);
        assert!(sustain >= 0. && sustain <= 1., "sustain was: {}", sustain);
        assert!(release >= 0., "release was: {}", release);
        Adsr { attack, decay, sustain, release }
    }

    pub fn apply(&self, elapsed: Seconds, elapsed_since_release: Seconds, sample: Sample) -> Sample {
        sample * self.scale_ratio(elapsed, elapsed_since_release)
    }

    fn scale_ratio(&self, elapsed: Seconds, elapsed_since_release: Seconds) -> Proportion {
        if elapsed_since_release > 0. {
            let release_progress = elapsed_since_release / self.release;
            let release_scale = (1. - release_progress).max(0.);
            self.sustain * release_scale
        } else if elapsed < self.attack {
            (elapsed / self.attack)
        } else if elapsed < self.attack + self.decay {
            let decay_progress = (elapsed - self.attack) / self.decay;
            let sustain_head_room = 1. - self.sustain;
            self.sustain + sustain_head_room * (1. - decay_progress)
        } else {
            self.sustain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Adsr;

    fn sut() -> Adsr {
        Adsr::new(1.,  1., 0.5, 1.)
    }

    #[test]
    fn attack() {
        assert_approx(sut().apply(0.,   0.,    0.1), 0.);
        assert_approx(sut().apply(0.25, 0.,    0.1), 0.025);
        assert_approx(sut().apply(0.5,  0.,    0.1), 0.05);
        assert_approx(sut().apply(0.75, 0.,    0.1), 0.075);
    }
    #[test]
    fn decay() {
        assert_approx(sut().apply(1.,   0.,    0.1), 0.1);
        assert_approx(sut().apply(1.25, 0.,    0.1), 0.0875);
        assert_approx(sut().apply(1.5,  0.,    0.1), 0.075);
        assert_approx(sut().apply(1.75, 0.,    0.1), 0.0625);
    }
    #[test]
    fn sustain() {
        assert_approx(sut().apply(2.,   0.,    0.1), 0.05);
        assert_approx(sut().apply(3.,   0.,    0.1), 0.05);
        assert_approx(sut().apply(10.,  0.,    0.1), 0.05);
    }
    #[test]
    fn release() {
        assert_approx(sut().apply(3.,   0.25,  0.1), 0.0375);
        assert_approx(sut().apply(3.,   0.5,   0.1), 0.025);
        assert_approx(sut().apply(3.,   0.75,  0.1), 0.0125);
        assert_approx(sut().apply(3.5,  1.,    0.1), 0.);
        assert_approx(sut().apply(4.,   1.5,   0.1), 0.);
        assert_approx(sut().apply(10.,  10.,   0.1), 0.);
    }

    #[test]
    fn release_before_decay() {
        assert_approx(sut().apply(0.,   10.,   0.1), 0.);
        assert_approx(sut().apply(1.,   10.,   0.1), 0.);
        assert_approx(sut().apply(1.,   0.5,   0.1), 0.025);
    }

    fn assert_approx(left: f64, right: f64) {
        assert!((right - left).abs() < 0.0000000000000001)
    }
}