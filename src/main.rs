use oscilator_ctrl::Command;
use std::thread;
use std::sync::mpsc::channel;

mod audio_out;
mod keyboard_in;
mod wave_gen;
mod oscilator_ctrl;

type Sample = f32;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let sample_rate = format.sample_rate.0 as f32;

    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = channel::<Sample>();

    thread::spawn(move || audio_out::play(&device, &format, sig_in));
    thread::spawn(move || oscilator_ctrl::start(sample_rate, cmd_in, sig_out));
    keyboard_in::listen(cmd_out);
}
