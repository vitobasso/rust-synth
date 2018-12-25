extern crate num_traits;

pub mod controller;
pub mod pulse;
pub mod arpeggiator;
pub mod loop_recorder;

pub type Sample = f64;
pub type Millis = u64;
pub type Hz = f64;