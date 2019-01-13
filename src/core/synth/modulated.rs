
pub trait Modulated<T> {
    fn mod_param(&mut self, target: T) -> Option<&mut ModParam>;
}

pub struct ModParam {
    base: f64,
    mod_signal: f64,
    min: f64,
    range: f64,
}
impl ModParam {
    pub fn with_bounds(min: f64, max: f64) -> ModParam {
        let range = max - min;
        ModParam { base: 1., mod_signal: 0., min, range }
    }
    pub fn with_base(base: f64, min: f64, max: f64) -> ModParam {
        let range = max - min;
        let bounded_base = base.max(0.).min(1.);
        ModParam { base: bounded_base, mod_signal: 0., min, range }
    }
    pub fn set_base(&mut self, value: f64) {
        self.base = value.max(0.).min(1.);
    }
    pub fn set_signal(&mut self, value: f64) {
        self.mod_signal = value.max(0.).min(1.);
    }
    pub fn calculate(&self) -> f64 {
        let normalized = (1. - self.mod_signal) * self.base;
        normalized * self.range + self.min
    }
}
impl Default for ModParam {
    fn default() -> Self {
        ModParam { base: 1., mod_signal: 0., min: 0., range: 1. }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_and_down() {
        let mut sut = ModParam::with_bounds(0.,  100.);
        sut.set_signal(0.);
        assert_approx(sut.calculate(), 100.);
        sut.set_signal(0.01);
        assert_approx(sut.calculate(), 99.);
        sut.set_signal(0.5);
        assert_approx(sut.calculate(), 50.);
        sut.set_signal(0.99);
        assert_approx(sut.calculate(), 1.);
        sut.set_signal(1.);
        assert_approx(sut.calculate(), 0.);
    }

    #[test]
    fn out_of_bounds() {
        let mut sut = ModParam::with_bounds(0.,  1.);
        sut.set_signal(-1.);
        assert_approx(sut.calculate(), 1.);
        sut.set_signal(2.);
        assert_approx(sut.calculate(), 0.);
    }

    #[test]
    fn negative_min() {
        let mut sut = ModParam::with_bounds(-10.,  10.);
        sut.set_signal(0.);
        assert_approx(sut.calculate(), 10.);
        sut.set_signal(0.5);
        assert_approx(sut.calculate(), 0.);
        sut.set_signal(1.);
        assert_approx(sut.calculate(), -10.);
    }

    #[test]
    fn base_at_half() {
        let mut sut = ModParam::with_bounds(0.,  10.);
        sut.set_base(0.5);
        sut.set_signal(0.);
        assert_approx(sut.calculate(), 5.);
        sut.set_signal(0.5);
        assert_approx(sut.calculate(), 2.5);
        sut.set_signal(1.);
        assert_approx(sut.calculate(), 0.);
    }

    fn assert_approx(left: f64, right: f64) {
        assert!((right - left).abs() < 0.00000000000001)
    }
}