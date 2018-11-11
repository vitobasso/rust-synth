use std::sync::mpsc::{Receiver, SyncSender};
use instrument::{WaveGen, Instrument, Switch};
use pitches::Pitch;

pub enum Command {
    Osc1, Osc2, Osc3, Osc4, Osc5, Osc6, Osc7, Osc8, Osc9, Osc0,
    NoteOn(Pitch), NoteOff(Pitch),
    Transpose(i8),
}

type Sample = f32;

pub fn run_forever(sample_rate: f32, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {

    let mut note_on: bool = false;
    let mut instrument = Instrument {
        pitch: Pitch::default(),
        osc: Switch { is_saw: false }
    };
    let mut transpose: i8 = 0_i8;
    let mut clock: f32 = 0.0;
    loop {
        match cmd_in.try_recv() {
            Ok(Command::Osc1) => {
                instrument.osc.is_saw = false;
            },
            Ok(Command::Osc2) => {
                instrument.osc.is_saw = true;
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
            _ => (),
        }

        clock = (clock + 1.0) % sample_rate;
        if note_on {
            let normalized_clock: f32 = clock/sample_rate;
            let sample: f32 = instrument.next_sample(normalized_clock);
            signal_out.send(sample).expect("Failed to send a sample");
        }
    }
}

