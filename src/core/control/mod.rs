extern crate num_traits;

pub mod controller;
pub mod pulse;
pub mod arpeggiator;
pub mod loops;
pub mod duration_recorder;
pub mod instrument_player;

pub type Sample = f64;
pub type Millis = u64;
pub type Hz = f64;