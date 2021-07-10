use super::diatonic_scale::{RelativePitch, OctaveShift, ScaleDegree};
use num_traits::FromPrimitive;

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive)]
pub enum NoteDuration {
    Whole=16, Half=8, Quarter=4, Eight=2, Sixteenth=1
}

impl NoteDuration {

    pub fn half(&self) -> Option<NoteDuration> {
        FromPrimitive::from_u8(*self as u8 / 2)
    }

    pub fn double(&self) -> Option<NoteDuration> {
        FromPrimitive::from_u8(*self as u8 * 2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Note {
    pub duration: NoteDuration,
    pub pitch: RelativePitch,
}

impl Note {
    pub fn new(duration: NoteDuration, octave: OctaveShift, degree: ScaleDegree) -> Self {
        Note { duration, pitch: (octave, degree) }
    }
}

impl Default for NoteDuration {
    fn default() -> Self {
        NoteDuration::Quarter
    }
}