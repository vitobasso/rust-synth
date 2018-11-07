use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};
use wave_gen::{Switch, WaveGen};

pub enum Command {
    Up,
    Down,
}

type Sample = f32;

pub fn start(sample_rate: f32, cmd_in: Receiver<Command>, signal_out: Sender<Sample>) {

    let mut osc = Switch{ is_saw: false};
    let mut clock: f32 = 0.0;
    loop {
        match cmd_in.try_recv() {
            Ok(Command::Up) => {
                osc.is_saw = true;
            },
            Ok(Command::Down) => {
                osc.is_saw = false;
            },
            _ => (),
        }

        clock = (clock + 1.0) % sample_rate;
        let normalized_clock: f32 = clock/sample_rate;
        let sample: f32 = osc.next_sample(normalized_clock);
        signal_out.send(sample).expect("Failed to send a sample");

        thread::sleep(Duration::from_nanos(11000)); //prevent sending faster than output can handle
    }
}

