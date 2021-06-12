use num_traits;

pub mod pitch;
pub mod diatonic_scale;
pub mod rhythm;

pub type Semitones = i8;
pub type Octave = i8;
pub type Hz = f64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Modality {
    MAJOR, MINOR
}

impl Default for Modality {
    fn default() -> Self {
        Modality::MINOR
    }
}