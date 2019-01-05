extern crate num_traits;

pub mod instrument;
pub mod oscillator;
pub mod filter;
pub mod envelope;
pub mod builder;
pub mod lfo;
pub mod modulated;

pub type Sample = f64;
pub type Seconds = f64;
pub type Proportion = f64;