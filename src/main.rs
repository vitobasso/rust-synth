#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;

use std::thread;
use std::sync::mpsc::{channel, sync_channel};
use controller::Command;

mod audio_out;
mod instrument;
mod pitches;
mod controller;
mod gui;

type Sample = f64;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let sample_rate = format.sample_rate.0 as f64;
    let buffer_size = sample_rate as usize / 250;

    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<Sample>(buffer_size);

    thread::spawn(move || audio_out::run_forever(&device, &format, sig_in));
    thread::spawn(move || controller::run_forever(sample_rate, cmd_in, sig_out));
    gui::window::show(cmd_out);
}
