extern crate num_traits;

pub mod manual_controller;
pub mod instrument_player;
pub mod transposer;
pub mod arpeggiator;
pub mod duration_recorder;
pub mod pulse;
pub mod loops;
pub mod playback_controller;
pub mod song;

pub type Millis = u64;
pub type Hz = f64;