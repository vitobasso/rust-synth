extern crate num_traits;

pub mod instrument;
pub mod pitch;
pub mod oscillator;
pub mod filter;
pub mod diatonic_scale;
pub mod rhythm;
pub mod pulse;
pub mod arpeggiator;
pub mod modulation;

pub type Sample = f64;
pub type Seconds = f64;
pub type Hz = f64;