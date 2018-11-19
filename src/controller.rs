use std::sync::mpsc::{Receiver, SyncSender};
use synth::{
    WaveGen, Instrument,
    oscillator::{Sine, Saw, Mix},
    filter::{BiquadFilter}
};
use pitches::Pitch;

pub enum Command {
    Osc1, Osc2, Osc3, Osc4, Osc5, Osc6, Osc7, Osc8, Osc9, Osc0,
    NoteOn(Pitch), NoteOff(Pitch),
    Transpose(i8),
    ModParam1(f64), ModParam2(f64),
}

type Sample = f64;

pub fn run_forever(sample_rate: f64, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {

    let mut note_on: bool = false;
    let mut instrument = Instrument::new(
        sample_rate,
        Box::new(Mix::supersaw(8, 3.0)),
        Box::new(BiquadFilter::lpf()),
    );
    let mut transpose: i8 = 0_i8;
    let mut clock: f64 = 0.0;
    loop {
        match cmd_in.try_recv() {
            Ok(Command::Osc1) => {
                instrument.oscillator = Box::new(Sine)
            },
            Ok(Command::Osc2) => {
                instrument.oscillator = Box::new(Saw)
            },
            Ok(Command::Osc3) => {
                instrument.oscillator = Box::new(Mix::supersaw(8, 3.0))
            },
            Ok(Command::NoteOn(pitch)) => {
                instrument.pitch = pitch + transpose;
                note_on = true;
            },
            Ok(Command::NoteOff(pitch)) => {
                if instrument.pitch == pitch + transpose {
                    note_on = false;
                }
            },
            Ok(Command::Transpose(n)) => {
                transpose = transpose + n;
            },
            Ok(Command::ModParam1(value)) => {
                instrument.set_mod_1(value);
            }
            Ok(Command::ModParam2(value)) => {
                instrument.set_mod_2(value);
            }
            _ => (),
        }

        clock = clock + 1.0;
        if note_on {
            let normalized_clock: f64 = clock/sample_rate;
            let sample: f64 = instrument.next_sample(normalized_clock);
            signal_out.send(sample).expect("Failed to send a sample");
        }
    }
}
