use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;

use crate::core::{control::{tools, sheet_music}};
use crate::core::synth::Sample;
use crate::io::audio::Out;

pub mod midi;
pub mod audio;

pub fn start_audio() -> (SyncSender<Sample>, f64){
    let out = Out::initialize().unwrap_or_else(|e| panic!("Failed to initialize audio: {}", e));
    let sample_rate = out.sample_rate();
    let (sound_out, sound_in) = mpsc::sync_channel::<Sample>(out.buffer_size());
    thread::spawn(move || out.start(sound_in));
    (sound_out, sample_rate)
}

pub fn start_manual() -> (Sender<tools::Command>, Receiver<tools::View>) {
    let (sound_out, sample_rate) = start_audio();
    let (command_out, command_in) = mpsc::channel::<tools::Command>();
    let (view_out, view_in) = mpsc::sync_channel::<tools::View>(1);
    thread::spawn(move || tools::start(sample_rate, command_in, sound_out, view_out));
    (command_out, view_in)
}

pub fn start_midi(file_path: &str) {
    let (sound_out, sample_rate) = start_audio();
    let music = midi::read_file(file_path)
        .unwrap_or_else(|| panic!("Failed to load MIDI file: [{}]", file_path));
    thread::spawn(move || sheet_music::start(sample_rate, music, sound_out));
}
