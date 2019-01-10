use std::sync::mpsc::{Receiver, SyncSender};
use core::{
    control::{Millis, arpeggiator::Arpeggiator, loops, duration_recorder::DurationRecorder,
              instrument_player::{self as player, Command::*}, transposer},
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

        state.arpeggiator.as_mut()
            .map( |arp| arp.next()).unwrap_or(vec!())
            .into_iter().for_each(|cmd|
                state.play_transposed(cmd));

        let new_sample = state.player.next_sample();
        let mix_sample = state.loops.next_sample() + new_sample;
        signal_out.send(mix_sample).expect("Failed to send a sample");

        state.loops.write(new_sample);
    }
}

pub type Id = u32;
pub enum Command {
    Instrument(player::Command),
    Transposer(transposer::Command),
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
    player: player::State,
    transposer: transposer::State,
    patches: Vec<Patch>,
    beat: Millis,
    arpeggiator: Option<Arpeggiator>,
    duration_recorder: DurationRecorder,
    loops: loops::Manager,
}

impl State {
    pub fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        State {
            patches,
            player: player::State::new(sample_rate),
            transposer: transposer::State::new(PitchClass::C),
            duration_recorder: DurationRecorder::new(),
            beat: DEFAULT_BEAT,
            arpeggiator: None,
            loops: loops::Manager::new(),
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(cmd) => self.play_or_arpeggiate(cmd),
            Command::Transposer(cmd) => self.transposer.interpret(cmd),
            Command::SetPatch(i) => self.set_patch(i),
            Command::Loop(cmd) => self.loops.interpret(cmd),
            Command::PulseRecord => self.record_beat(),
        }
    }

    fn play_or_arpeggiate(&mut self, command: player::Command) {
        match command {
            NoteOn(_, _) | NoteOff(_) =>  {
                if let Some(arp) = &mut self.arpeggiator {
                    arp.interpret(command);
                }
                if self.arpeggiator.is_none() {
                    self.play_transposed(command);
                }
            },
            _ => self.play_transposed(command)
        }
    }

    fn play_transposed(&mut self, command: player::Command) {
        let changed_command = match command {
            NoteOn(pitch, id) => NoteOn(self.transposer.transpose(pitch), id),
            other => other,
        };
        self.player.interpret(changed_command)
    }

    fn set_patch(&mut self, i: usize) {
        let patch: Patch = self.patches.get(i).cloned().unwrap_or(Patch::Noop);
        match patch {
            Patch::Instrument(specs) => self.player.set_instrument(specs),
            Patch::Oscillator(specs) => self.player.set_oscillator(specs),
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
