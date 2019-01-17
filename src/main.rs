#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[macro_use] extern crate num_derive;

mod io;
pub mod core;
pub mod preset;
mod gui;

use std::{thread, sync::mpsc::{channel, sync_channel}};
use crate::core::{music_theory::Hz, synth::Sample,
           control::{manual_controller::{self, Command}, playback_controller}};
use crate::io::audio_out;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let sample_rate = Hz::from(format.sample_rate.0);
    let buffer_size = sample_rate as usize / 250;

    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<Sample>(buffer_size);

    thread::spawn(move || audio_out::loop_forever(&device, &format, sig_in));
    match read_midi_file() {
        Some(song) => thread::spawn(move || playback_controller::loop_forever(sample_rate, song, sig_out)),
        None       => thread::spawn(move || manual_controller::loop_forever(sample_rate, preset::patches(), cmd_in, sig_out)),
    };

    gui::window::show(cmd_out);
}

use crate::core::control::song::Song;
use std::env::{args,Args};
fn read_midi_file() -> Option<Song> {
    let mut args: Args = args();
    args.next();
    args.next().and_then(|file_name| io::midi::read_file(file_name.as_str()))
}
