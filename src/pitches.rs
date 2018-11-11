
#[derive(PartialEq, Clone)]
pub enum PitchClass {
    C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B
}

impl PitchClass {
    fn from_index(i: u8) -> PitchClass {
        match i {
            0 => PitchClass::C,
            1 => PitchClass::Db,
            2 => PitchClass::D,
            3 => PitchClass::Eb,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::Gb,
            7 => PitchClass::G,
            8 => PitchClass::Ab,
            9 => PitchClass::A,
            10 => PitchClass::Bb,
            11 => PitchClass::B,
            _ => panic!("Can't get PitchClass from index: {}.", i), //TODO type safely?
        }
    }
}

#[derive(PartialEq)]
pub struct Pitch {
    pub class: PitchClass,
    pub octave: u8,
}

impl Pitch {

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
    pub fn freq(&self) -> f32 {
        let f0: f64 = 16.35_f64;
        let a: f64 = 2_f64.powf(1_f64/12_f64);
        (f0 * a.powf(self.index() as f64)) as f32
    }

    fn index(&self) -> u8{
        self.octave * 12 + self.class.clone() as u8
    }

    fn from_index(i: u8) -> Pitch {
        Pitch {
            octave: i/12,
            class: PitchClass::from_index(i%12)
        }
    }

}

impl Default for Pitch {
    fn default() -> Self {
        Pitch { class: PitchClass::A, octave: 4 }
    }
}

use std::ops::Add;
impl Add<i8> for Pitch {
    type Output = Self;
    fn add(self, rhs: i8) -> Self {
        Pitch::from_index((self.index() as i8 + rhs) as u8)
    }
}


#[cfg(test)]
mod tests {
    use pitches::{Pitch, PitchClass::*};
    #[test]
    fn should_convert_pitch_to_freq() {
        let cases: &[(Pitch, f32)] = &[
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