use std::sync::mpsc::{Sender, SyncSender};
use std::sync::mpsc;
use std::thread;

use crate::core::control::{manual_controller, playback_controller};
use crate::core::control::manual_controller::Command;
use crate::core::synth::Sample;
use crate::io::audio::Out;
use crate::preset;

pub mod midi;
pub mod audio;

pub fn start_audio() -> (SyncSender<Sample>, f64){
    let out = Out::initialize().unwrap_or_else(|e| panic!("Failed to initialize audio: {}", e));
    let sample_rate = out.sample_rate();
    let (sound_out, sound_in) = mpsc::sync_channel::<Sample>(out.buffer_size());
    thread::spawn(move || out.start(sound_in));
    (sound_out, sample_rate)
}

pub fn start_manual() -> Sender<Command>{
    let (sound_out, sample_rate) = start_audio();
    let (command_out, command_in) = mpsc::channel::<Command>();
    thread::spawn(move || manual_controller::start(sample_rate, preset::patches(), command_in, sound_out));
    command_out
}

pub fn start_midi(file_path: &str) {
    let (sound_out, sample_rate) = start_audio();
    let song= midi::read_file(file_path)
        .unwrap_or_else(|| panic!("Failed to load MIDI file: [{}]", file_path));
    thread::spawn(move || playback_controller::start(sample_rate, song, sound_out));
}
