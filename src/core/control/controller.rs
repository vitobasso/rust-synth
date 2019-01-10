use std::sync::mpsc::{Receiver, SyncSender};
use core::{
    control::{Millis, arpeggiator::Arpeggiator, loops, duration_recorder::DurationRecorder,
              instrument_player},
    music_theory::{Hz, pitch::PitchClass, rhythm::Sequence},
    synth::{Sample, instrument, oscillator},
};

const PULSES_PER_BEAT: u64 = 4;
const DEFAULT_BEAT: Millis = 100 * PULSES_PER_BEAT;

pub fn run_forever(sample_rate: Hz, patches: Vec<Patch>, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate, patches);
    loop {
        match cmd_in.try_recv() {
            Ok(command) => state.interpret(command),
            _ => (),
        }

        if let Some(arp) = state.arpeggiator.as_mut() {
            let player = &mut state.instrument_player;
             arp.next().into_iter().for_each(|cmd|
                player.interpret(cmd))
        }

        let new_sample = state.instrument_player.next_sample();
        let mix_sample = state.loops.next_sample() + new_sample;
        signal_out.send(mix_sample).expect("Failed to send a sample");

        state.loops.write(new_sample);
    }
}

pub type Id = u32;
pub enum Command {
    Instrument(instrument_player::Command),
    SetPatch(usize),
    Loop(loops::Command),
    PulseRecord,
}

#[derive(Clone)]
pub enum Patch {
    Instrument(instrument::Specs),
    Oscillator(oscillator::Specs),
    Arpeggiator(Option<Sequence>),
    Noop,
}

struct State {
    instrument_player: instrument_player::State,
    patches: Vec<Patch>,
    beat: Millis, duration_recorder: DurationRecorder,
    arpeggiator: Option<Arpeggiator>,
    loops: loops::Manager,
}

impl State {
    pub fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        State {
            patches,
            instrument_player: instrument_player::State::new(sample_rate),
            duration_recorder: DurationRecorder::new(),
            beat: DEFAULT_BEAT,
            arpeggiator: None,
            loops: loops::Manager::new(),
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(cmd) => self.insterpret_instrument_cmd(cmd),
            Command::SetPatch(i) => self.set_patch(i),
            Command::Loop(cmd) => self.loops.interpret(cmd),
            Command::PulseRecord => self.record_beat(),
        }
    }

    fn insterpret_instrument_cmd(&mut self, command: instrument_player::Command) {
        let arpeggiator = &mut self.arpeggiator;
        let player = &mut self.instrument_player;
        match command {
            instrument_player::Command::NoteOn(_, _)
            | instrument_player::Command::NoteOff(_, _) =>  {
                match arpeggiator {
                    Some(arp) => arp.interpret(command),
                    None => player.interpret(command)
                }
            },
            _ => player.interpret(command)
        }
    }

    fn set_patch(&mut self, i: usize) {
        let patch: Patch = self.patches.get(i).cloned().unwrap_or(Patch::Noop);
        match patch {
            Patch::Instrument(specs) => self.instrument_player.set_instrument(specs),
            Patch::Oscillator(specs) => self.instrument_player.set_oscillator(specs),
            Patch::Arpeggiator(seq) => self.set_arpeggiator(seq),
            Patch::Noop => (),
        }
    }

    fn set_arpeggiator(&mut self, seq: Option<Sequence>) {
        self.arpeggiator = seq.map(|s|
            Arpeggiator::new(self.get_pulse_millis(), PitchClass::C, s));
    }

    fn record_beat(&mut self) {
        self.duration_recorder.record();
        if let Some(duration) = self.duration_recorder.read() {
            self.beat = duration;
            let pulse = self.get_pulse_millis().clone();
            if let Some(arp) = &mut self.arpeggiator {
                arp.set_pulse(pulse)
            }
        }
    }

    fn get_pulse_millis(&self) -> Millis {
        self.beat / PULSES_PER_BEAT
    }

}
