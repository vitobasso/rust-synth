use std::ops::{Add, Sub};
use super::{num_traits::FromPrimitive, Octave, pitch::Pitch, pitch_class::PitchClass::{self, *}};
use self::ScaleDegree::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ScaleDegree {
    I1, I2, I3, I4, I5, I6, I7
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive)]
pub enum OctaveShift {
    Down3=-3, Down2=-2, Down1=-1, Same=0, Up1=1, Up2=2, Up3=3
}
pub type RelativePitch = (OctaveShift, ScaleDegree);

impl Add<ScaleDegree> for ScaleDegree {
    type Output = Self;
    fn add(self, rhs: ScaleDegree) -> Self {
        let i = (self as u8 + rhs as u8) % 7;
        FromPrimitive::from_u8(i)
            .unwrap_or_else(|| panic!("Failed to get ScaleDegree for i={}", i))
    }
}
impl Sub<ScaleDegree> for ScaleDegree {
    type Output = Self;
    fn sub(self, rhs: ScaleDegree) -> Self {
        let i = ((((self as i8 - rhs as i8) % 7) + 7) % 7) as u8;
        FromPrimitive::from_u8(i)
            .unwrap_or_else(|| panic!("Failed to get ScaleDegree for i={}", i))
    }
}

pub type Key = PitchClass;
impl Key {
    pub fn pitch_at(self, offset: Pitch, interval: RelativePitch) -> Option<Pitch> {
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

    pub fn transpose_to(self, other: Key, pitch: Pitch) -> Option<Pitch> {
        self.transpose_class_to(other, pitch.class).map(|class| {
            let carry_octave =
                if pitch.class == PitchClass::C && class == PitchClass::B {-1} else {0};
            Pitch::new(class, pitch.octave + carry_octave)
        })
    }

    fn transpose_class_to(self, other: Key, pitch_class: PitchClass) -> Option<PitchClass> {
        self.degree_of(pitch_class).map(|degree|
            match (self.degree_of(other), other.degree_of(self)) {
                (Some(key_diff), _) => other.pitch_class_at(degree - key_diff),
                (None, Some(reciprocal_key_diff)) => other.pitch_class_at(degree + reciprocal_key_diff),
                (None, None) => if degree == I4 { pitch_class } else { pitch_class - 1 }
            }
        )
    }

    pub fn shift_fifths(self, increment: i8) -> Key {
        self + (7 * increment) % 12
    }

    pub fn distance_fifths(self, other: Key) -> i8 {
        let norm_self = self - C;
        let norm_other_usize = (other - norm_self) as usize % 12;
        let norm_other = PitchClass::from_index(norm_other_usize)
            .unwrap_or_else(|| panic!("Expected int < 12"));
        match norm_other {
            C => 0,
            G => 1,
            D => 2,
            A => 3,
            E => 4,
            B => 5,
            Gb => 6,
            Db => -5,
            Ab => -4,
            Eb => -3,
            Bb => -2,
            F => -1,
        }
    }

}


#[cfg(test)]
mod tests {
    use super::{Key, OctaveShift::*, Pitch,
                PitchClass::{self, *}, RelativePitch, ScaleDegree::{self, *}};

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
        let cases: &[(Key, Pitch, RelativePitch, Option<Pitch>)] = &[
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
    fn shift_fifths() {
        let cases: &[(Key, i8, Key)] = &[
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
            let actual_result = key.shift_fifths(*increment);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}", *key, *increment, *expected_result);
        }
    }

    #[test]
    fn distance_fifths() {
        let cases: &[(Key, Key, i8)] = &[
            (C,  C,  0),
            (C,  G,  1),
            (C,  D,  2),
            (C,  A,  3),
            (C,  E,  4),
            (C,  B,  5),
            (C,  Gb, 6),
            (C,  Db, -5),
            (C,  Ab, -4),
            (C,  Eb, -3),
            (C,  Bb, -2),
            (C,  F,  -1),
            (G,  C,  -1),
            (G,  D,  1),
        ];
        for (key, other, expected_result) in cases.iter() {
            let actual_result = key.distance_fifths(*other);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}", *key, *other, *expected_result);
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
            (C,  Gb, C,  Some(B)),
            (C,  Gb, F,  Some(F)),
            (C,  G,  Db, None),
        ];
        for (source_key, target_key, pitch_class, expected_result) in cases.iter() {
            let actual_result = source_key.transpose_class_to(*target_key, *pitch_class);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}, {:?}", *source_key, *target_key, *pitch_class, *expected_result);
        }
    }


    #[test]
    fn transpose_to() {
        let cases: &[(Key, Key, Pitch, Option<Pitch>)] = &[
            (C,  G,   Pitch{class: C, octave: 4},  Some(Pitch{class: C, octave: 4})),
            (C,  Gb,  Pitch{class: D, octave: 4},  Some(Pitch{class: Db, octave: 4})),
            (C,  Gb,  Pitch{class: C, octave: 4},  Some(Pitch{class: B, octave: 3})),
        ];
        for (source_key, target_key, pitch, expected_result) in cases.iter() {
            let actual_result = source_key.transpose_to(*target_key, *pitch);
            assert_eq!(actual_result, *expected_result,
                       "Input was: {:?}, {:?}, {:?}, {:?}", *source_key, *target_key, *pitch, *expected_result);
        }
    }

}