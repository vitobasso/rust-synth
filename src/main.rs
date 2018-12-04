#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[macro_use] extern crate num_derive;

use std::thread;
use std::sync::mpsc::{channel, sync_channel};
use controller::Command;
use controller::StateUpdate;

mod audio_out;
pub mod synth; //TODO pub is temporary to stop dead code wornings
mod controller;
mod gui;

type Sample = f64;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let sample_rate = format.sample_rate.0 as f64;
    let buffer_size = sample_rate as usize / 250;

    let (cmd_send, cmd_recv) = channel::<Command>();
    let (sig_send, sig_recv) = sync_channel::<Sample>(buffer_size);
    let (upd_send, upd_recv) = channel::<StateUpdate>();

    thread::spawn(move || audio_out::run_forever(&device, &format, sig_recv));
    thread::spawn(move || controller::run_forever(sample_rate, cmd_recv, sig_send, upd_send));
    gui::window::show(cmd_send, upd_recv);
}
