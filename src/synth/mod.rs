extern crate num_traits;

pub mod controller;
pub mod instrument;
pub mod pitch;
pub mod oscillator;
pub mod filter;
pub mod diatonic_scale;
pub mod rhythm;
pub mod pulse;
pub mod arpeggiator;
pub mod envelope;
pub mod loop_recorder;

pub type Sample = f64;
pub type Seconds = f64;
pub type Millis = u64;
pub type Hz = f64;
pub type ScaleRatio = f64;