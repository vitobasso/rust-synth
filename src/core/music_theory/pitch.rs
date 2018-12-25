use std::ops::{Add, Sub};
use super::{Hz, Semitones, Octave, num_traits::FromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, Debug)]
pub enum PitchClass {
    C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B
}

impl PitchClass {
    pub fn from_index(i: usize) -> Option<PitchClass> {
        FromPrimitive::from_usize(i)
    }
}

impl Add<Semitones> for PitchClass {
    type Output = Self;
    fn add(self, rhs: Semitones) -> Self {
        let i = ((((self as Semitones + rhs) % 12) + 12) % 12) as usize;
        PitchClass::from_index(i).expect(format!("Failed to get PitchClass for i={}", i).as_str())
    }
}
impl Add<PitchClass> for PitchClass {
    type Output = Self;
    fn add(self, rhs: PitchClass) -> Self {
        self + rhs as Semitones
    }
}
impl Sub<Semitones> for PitchClass {
    type Output = Self;
    fn sub(self, rhs: Semitones) -> Self {
        self + -rhs
    }
}
impl Sub<PitchClass> for PitchClass {
    type Output = Self;
    fn sub(self, rhs: PitchClass) -> Self {
        self - rhs as Semitones
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pitch {
    pub class: PitchClass,
    pub octave: Octave,
}

impl Pitch {

    pub fn new(class: PitchClass, octave: Octave) -> Pitch {
        Pitch { class, octave }
    }

    /// Frequencies on the equal tempered scale:
    ///
    ///     fn = f0 * (a)^n
    ///
    /// where
    /// f0 = the frequency of one fixed note which must be defined. A common choice is setting the A above middle C (A4) at f0 = 440 Hz.
    /// n = the number of half steps away from the fixed note you are. If you are at a higher note, n is positive. If you are on a lower note, n is negative.
    /// fn = the frequency of the note n half steps away.
    /// a = (2)1/12 = the twelth root of 2 = the number which when multiplied by itself 12 times equals 2 = 1.059463094359...
    ///
    /// Source: http://pages.mtu.edu/~suits/NoteFreqCalcs.html
    ///
    pub fn freq(&self) -> Hz {
        let f0: Hz = 16.35_f64;
        let a: f64 = 2_f64.powf(1_f64/12_f64);
        (f0 * a.powf(self.index() as f64))
    }

    fn index(&self) -> usize{
        (self.octave * 12 + self.class.clone() as i8) as usize
    }

    fn from_index(i: usize) -> Pitch {
        Pitch {
            octave: (i/12) as Octave,
            class: PitchClass::from_index(i%12)
                .expect(format!("Failed to get PitchClass for i={}", i).as_str())
        }
    }

}

impl Default for Pitch {
    fn default() -> Self {
        Pitch { class: PitchClass::A, octave: 4 }
    }
}

impl Add<Semitones> for Pitch {
    type Output = Self;
    fn add(self, rhs: Semitones) -> Self {
        Pitch::from_index((self.index() as Semitones + rhs) as usize)
    }
}


#[cfg(test)]
mod tests {
    use super::{Pitch, PitchClass::*, Hz};
    #[test]
    fn should_convert_pitch_to_freq() {
        let cases: &[(Pitch, Hz)] = &[
            (Pitch{ octave: 0, class: C }, 16.35  ),
            (Pitch{ octave: 4, class: C }, 261.63 ),
            (Pitch{ octave: 4, class: A }, 440.0  ),
            (Pitch{ octave: 5, class: A }, 880.0  ),
            (Pitch{ octave: 8, class: B }, 7902.13),
        ];
        for (pitch, expected_freq) in cases.iter() {
            let err = expected_freq - pitch.freq();
            assert!(err.abs() < 1.0);
        }
    }
}