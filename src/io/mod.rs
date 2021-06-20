use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::thread;

use crate::core::{control::{tools, tools::Command, sheet_music}};
use crate::io::audio::AudioOut;
use crate::preset;

pub mod midi;
pub mod audio;

pub fn start_audio() -> AudioOut {
    AudioOut::initialize().unwrap_or_else(|e| panic!("Failed to initialize audio: {}", e))
}

pub fn start_manual() -> Sender<Command>{
    let (command_out, command_in) = mpsc::channel::<Command>();
    thread::spawn(move || tools::start(preset::patches(), command_in, start_audio()));
    command_out
}

pub fn start_midi(file_path: &str) {
    let music = midi::read_file(file_path)
        .unwrap_or_else(|| panic!("Failed to load MIDI file: [{}]", file_path));
    thread::spawn(move || sheet_music::start(music, start_audio()));
}
