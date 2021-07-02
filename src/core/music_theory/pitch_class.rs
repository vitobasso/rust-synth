use std::ops::{Add, Sub, AddAssign};
use super::{Semitones, num_traits::FromPrimitive};
use std::fmt::{Display, Formatter};

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

impl Display for PitchClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
