#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[macro_use] extern crate num_derive;

mod io;
pub mod core;
pub mod preset;
mod gui;

use std::{thread, sync::mpsc::{channel, sync_channel}};
use crate::core::{synth::Sample, control::{manual_controller::{self, Command}, playback_controller}};
use crate::io::{audio, midi};

fn main() {
    let out = audio::Out::initialize().unwrap_or_else(|e| panic!(e));
    let sample_rate = out.sample_rate();

    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<Sample>(out.buffer_size());

    thread::spawn(move || out.loop_forever(sig_in));
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
    args.next().and_then(|file_name| midi::read_file(file_name.as_str()))
}
