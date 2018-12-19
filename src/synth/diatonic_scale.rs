use super::pitch::{Pitch, PitchClass::{self, *}, Octave, Semitones};
use self::ScaleDegree::*;
use std::ops::{Add, Sub};
use super::num_traits::FromPrimitive;

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
pub enum ScaleDegree {
    I1, I2, I3, I4, I5, I6, I7
}
#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
pub enum OctaveShift {
    Down3=-3, Down2=-2, Down1=-1, Same=0, Up1=1, Up2=2, Up3=3
}
pub type ScalePitch = (OctaveShift, ScaleDegree);

impl Add<ScaleDegree> for ScaleDegree {
    type Output = Self;
    fn add(self, rhs: ScaleDegree) -> Self {
        let i = (self as u8 + rhs as u8) % 7;
        FromPrimitive::from_u8(i)
            .expect(&format!("Failed to get ScaleDegree for i={}", i))
    }
}
impl Sub<ScaleDegree> for ScaleDegree {
    type Output = Self;
    fn sub(self, rhs: ScaleDegree) -> Self {
        let i = ((((self as i8 - rhs as i8) % 7) + 7) % 7) as u8;
        FromPrimitive::from_u8(i)
            .expect(&format!("Failed to get ScaleDegree for i={}", i))
    }
}

pub type Key = PitchClass;
impl Key {
    pub fn pitch_at(self, offset: Pitch, interval: ScalePitch) -> Option<Pitch> {
        self.degree_of(offset.class).and_then(|offset_degree: ScaleDegree| {
            let (octave_increment, degree_increment) = interval;
            let new_degree: ScaleDegree = offset_degree + degree_increment;
            let new_class: PitchClass = self.pitch_class_at(new_degree);
            let carry_octave: Octave = if (new_class as u8) < offset.class as u8 {1} else {0};
            let new_octave: Octave = offset.octave as i8 + octave_increment as i8 + carry_octave;
            if new_octave > 0 {
                Some(Pitch::new(new_class, new_octave))
            } else {
                None
            }
        })
    }
    fn degree_of(self, pitch_class: PitchClass) -> Option<ScaleDegree> {
        let relative_pitch = pitch_class - self;
        match relative_pitch {
            C => Some(I1),
            D => Some(I2),
            E => Some(I3),
            F => Some(I4),
            G => Some(I5),
            A => Some(I6),
            B => Some(I7),
            _ => None
        }
    }
    fn pitch_class_at(self, degree: ScaleDegree) -> PitchClass {
        let relative_pitch = match degree {
            I1 => C,
            I2 => D,
            I3 => E,
            I4 => F,
            I5 => G,
            I6 => A,
            I7 => B,
        };
        self + relative_pitch
    }

    pub fn transpose_to(&self, other: Key, pitch: Pitch) -> Option<Pitch> {
        self.transpose_class_to(other, pitch.class).map(|class| Pitch::new(class, pitch.octave))
    }

    fn transpose_class_to(&self, other: Key, pitch_class: PitchClass) -> Option<PitchClass> {
        match self.degree_of(other) {
            Some(key_distance) =>
                self.degree_of(pitch_class).map(|degree| {
                    other.pitch_class_at(degree - key_distance)
                }),
            None => other.transpose_class_from(*self, pitch_class)
        }
    }

    fn transpose_class_from(&self, other: Key, pitch_class: PitchClass) -> Option<PitchClass> {
        match self.degree_of(other) {
            Some(key_distance) =>
                other.degree_of(pitch_class).map(|degree| {
                    self.pitch_class_at(degree + key_distance)
                }),
            None => panic!("Fundamental of Key {:?} is not in the scale of Key {:?}.", other, self)
        }
    }

    pub fn circle_of_fifths(&self, increment: Semitones) -> Key {
        *self + (7 * increment) % 12
    }

}


#[cfg(test)]
mod tests {
    use super::{PitchClass::{self, *}, Pitch, Key, Semitones,
                ScaleDegree::{self, *}, OctaveShift::*, ScalePitch };

    #[test]
    fn pitch_class_to_scale_degree() {
        let cases: &[(Key, PitchClass, Option<ScaleDegree>)] = &[
            (C,  C,  Some(I1)),
            (C,  Db, None),
            (C,  D,  Some(I2)),
            (C,  Eb, None),
            (C,  E,  Some(I3)),
            (C,  F,  Some(I4)),
            (C,  Gb, None),
            (C,  G,  Some(I5)),
            (C,  Ab, None),
            (C,  A,  Some(I6)),
            (C,  B,  Some(I7)),
            (Db, Db, Some(I1)),
            (Db, C,  Some(I7)),
            (Db, D,  None),
            (Db, C,  Some(I7)),
            (Db, D,  None),
        ];
        for (key, pitch, expected_result) in cases.iter() {
            let actual_result: Option<ScaleDegree> = key.degree_of(*pitch);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?} {:?} {:?}", key, pitch, *expected_result);
        }
    }

    #[test]
    fn scale_degree_to_pitch_class() {
        let cases: &[(Key, ScaleDegree, PitchClass)] = &[
            (C,   I1, C),
            (C,   I2, D),
            (C,   I3, E),
            (C,   I4, F),
            (C,   I5, G),
            (C,   I6, A),
            (C,   I7, B),
            (Db,  I1, Db),
            (Db,  I2, Eb),
            (Db,  I3, F),
            (Db,  I4, Gb),
            (Db,  I5, Ab),
            (Db,  I6, Bb),
            (Db,  I7, C),
        ];
        for (key, degree, expected_result) in cases.iter() {
            let actual_result = key.pitch_class_at(*degree);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?} {:?} {:?}", key, degree, *expected_result);
        }
    }

    #[test]
    fn scale_degree_to_pitch() {
        let c4 = Pitch::new(C, 4);
        let d4 = Pitch::new(D, 4);
        let a4 = Pitch::new(A, 4);
        let b4 = Pitch::new(B, 4);
        let cases: &[(Key, Pitch, ScalePitch, Option<Pitch>)] = &[
            (C,  c4, (Same,  I1), Some(c4)),
            (C,  a4, (Same,  I1), Some(a4)),
            (C,  b4, (Same,  I1), Some(b4)),
            (Bb, d4, (Same,  I1), Some(d4)),
            (D,  d4, (Same,  I1), Some(d4)),
            (C,  c4, (Up1,   I1), Some(Pitch::new(C,  5))),
            (C,  c4, (Down3, I1), Some(Pitch::new(C,  1))),
            (C,  c4, (Same,  I2), Some(Pitch::new(D,  4))),
            (C,  c4, (Same,  I7), Some(Pitch::new(B,  4))),
            (C,  c4, (Down1, I7), Some(Pitch::new(B,  3))),
            (C,  d4, (Same,  I2), Some(Pitch::new(E,  4))),
            (C,  a4, (Same,  I3), Some(Pitch::new(C, 5))),
            (Bb, d4, (Same,  I2), Some(Pitch::new(Eb, 4))),
            (D,  d4, (Same,  I2), Some(Pitch::new(E,  4))),
            (D,  d4, (Same,  I7), Some(Pitch::new(Db, 5))),
            (B,  c4, (Same,  I1), None), //offset is out of key
            (C,  Pitch::new(C, 0), (Down1,  I1), None), //min octave is 0
        ];
        for (key, offset, interval, expected_result) in cases.iter() {
            let actual_result = key.pitch_at(*offset, *interval);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}, {:?}", key, *offset, *interval, *expected_result);
        }
    }

    #[test]
    fn circle_of_fifths() {
        let cases: &[(Key, Semitones, Key)] = &[
            (C,  1,  G),
            (C,  2,  D),
            (C,  3,  A),
            (C,  4,  E),
            (C,  5,  B),
            (C,  6,  Gb),
            (C,  7,  Db),
            (C,  8,  Ab),
            (C,  9,  Eb),
            (C,  10, Bb),
            (C,  11, F),
            (C,  -1,  F),
            (C,  -2,  Bb),
        ];
        for (key, increment, expected_result) in cases.iter() {
            let actual_result = key.circle_of_fifths(*increment);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}", *key, *increment, *expected_result);
        }
    }

    #[test]
    fn transpose_class_to() {
        let cases: &[(Key, Key, PitchClass, Option<PitchClass>)] = &[
            (C,  G,  C,  Some(C)),
            (C,  G,  F,  Some(Gb)),
            (C,  F,  B,  Some(Bb)),
            (C,  E,  C,  Some(Db)),
            (C,  E,  D,  Some(Eb)),
            (C,  E,  E,  Some(E)),
            (C,  E,  F,  Some(Gb)),
            (C,  E,  G,  Some(Ab)),
            (C,  E,  A,  Some(A)),
            (C,  E,  B,  Some(B)),
            (C,  Bb, E,  Some(Eb)),
            (C,  C,  C,  Some(C)),
            (C,  G,  Db, None),
        ];
        for (key, other_key, pitch_class, expected_result) in cases.iter() {
            let actual_result = key.transpose_class_to(*other_key, *pitch_class);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}, {:?}", *key, *other_key, *pitch_class, *expected_result);
        }
    }

}