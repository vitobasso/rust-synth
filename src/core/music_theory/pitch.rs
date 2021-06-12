use std::ops::{Add, Sub, AddAssign};
use super::{Hz, Semitones, Octave, num_traits::FromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, Debug)]
pub enum PitchClass {
    C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B
}

pub const NUM_CLASSES: usize = 12;

impl PitchClass {
    pub fn from_index(i: usize) -> Option<PitchClass> {
        FromPrimitive::from_usize(i)
    }
}

impl Add<Semitones> for PitchClass {
    type Output = Self;
    fn add(self, rhs: Semitones) -> Self {
        let limit = NUM_CLASSES as i8;
        let i = ((((self as Semitones + rhs) % limit) + limit) % limit) as usize;
        PitchClass::from_index(i).unwrap_or_else(|| panic!("Failed to get PitchClass for i={}", i))
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
impl AddAssign<Semitones> for PitchClass {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs
    }
}

impl Default for PitchClass {
    fn default() -> Self {
        PitchClass::A
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    pub class: PitchClass,
    pub octave: Octave,
}

impl Pitch {

    pub fn new(class: PitchClass, octave: Octave) -> Pitch {
        Pitch { class, octave }
    }

    /// Frequencies on the equal tempered scale:
    ///     fn = f0 * (a)^n
    ///
    /// where
    /// f0 = the frequency of one fixed note which must be defined. A common choice is setting the A above middle C (A4) at f0 = 440 Hz.
    /// n = the number of half steps away from the fixed note you are. If you are at a higher note, n is positive. If you are on a lower note, n is negative.
    /// fn = the frequency of the note n half steps away.
    /// a = (2)1/12 = the twelfth root of 2 = the number which when multiplied by itself 12 times equals 2 = 1.059463094359...
    ///
    /// Source: http://pages.mtu.edu/~suits/NoteFreqCalcs.html
    ///
    pub fn freq(self) -> Hz {
        let f0: Hz = 440.;
        let a: f64 = 2_f64.powf(1./12.);
        let n: isize = self.index() as isize - 69;
        f0 * a.powf(n as f64)
    }

    /// Follows the MIDI convention: the index for C4 is 60
    /// https://newt.phys.unsw.edu.au/jw/notes.html
    pub fn index(self) -> usize{
        (self.octave + 1) as usize * NUM_CLASSES + self.class as usize
    }

    /// Follows the MIDI convention: the index for C4 is 60
    /// https://newt.phys.unsw.edu.au/jw/notes.html
    pub fn from_index(i: usize) -> Pitch {
        Pitch {
            octave: ((i / NUM_CLASSES) as Octave - 1),
            class: PitchClass::from_index(i % NUM_CLASSES)
                .unwrap_or_else(|| panic!("Failed to get PitchClass for i={}", i))
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

use std::fmt::{Debug, Formatter};
impl Debug for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.class, self.octave)
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

    /// Follows the MIDI convention: the index for C4 is 60
    /// https://newt.phys.unsw.edu.au/jw/notes.html
    #[test]
    fn should_convert_index_to_pitch() {
        let cases: &[(usize, Pitch)] = &[
            (21,        Pitch{ octave: 0, class: A }),
            (60,        Pitch{ octave: 4, class: C }),
            (69,        Pitch{ octave: 4, class: A }),
            (60 - 4*12, Pitch{ octave: 0, class: C }),
            (69 + 12,   Pitch{ octave: 5, class: A }),
            (59 + 5*12, Pitch{ octave: 8, class: B }),
        ];
        for (index, expected_pitch) in cases.iter() {
            assert_eq!(Pitch::from_index(*index), *expected_pitch);
        }
    }
}