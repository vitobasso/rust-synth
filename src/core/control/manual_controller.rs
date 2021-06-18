use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use crate::core::{
    control::{arpeggiator::Arpeggiator, duration_recorder::DurationRecorder,
              instrument_player::{self as player, Command::*}, loops, Millis,
              pulse::{Pulse, PulseReading}, transposer, song::MeasurePosition},
    music_theory::{Hz, pitch::PitchClass, rhythm::Phrase},
    synth::{instrument, oscillator, Sample},
};

pub fn start(sample_rate: Hz, presets: Vec<Patch>, command_in: Receiver<Command>, sound_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate, presets);
    loop {
        if let Ok(command) = command_in.try_recv() {
            state.interpret(command);
        }
        state.tick_arpeggiator();

        let new_sample = state.next_sample();
        sound_out.send(new_sample).expect("Failed to send a sample");
        state.loops.write(new_sample);
    }
}

const BEATS_PER_MEASURE: u64 = 4;
const PULSES_PER_BEAT: u64 = 32;
const DEFAULT_PULSE: Millis = 12;

#[derive(Clone, Copy)]
pub enum Command {
    Instrument(player::Command),
    Transposer(transposer::Command),
    SetPatchNo(usize),
    Loop(loops::Command),
    PulseRecord,
}

#[derive(Clone)]
pub enum Patch {
    Instrument(instrument::Specs),
    Oscillator(oscillator::Specs),
    Arpeggiator(Option<Phrase>),
    Noop,
}

struct State {
    player: player::State,
    transposer: transposer::State,
    patches: Vec<Patch>,
    pulse: Pulse,
    arpeggiator: Option<Arpeggiator>,
    arp_index: f64,
    duration_recorder: DurationRecorder,
    loops: loops::Manager,
}

impl State {
    fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        State {
            patches,
            player: player::State::with_default_specs(sample_rate),
            transposer: transposer::State::new(PitchClass::C),
            duration_recorder: Default::default(),
            pulse: Pulse::new_with_millis(DEFAULT_PULSE),
            arpeggiator: None,
            arp_index: 0.,
            loops: Default::default(),
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(cmd) => self.play_or_arpeggiate(cmd),
            Command::Transposer(cmd) => self.transposer.interpret(cmd),
            Command::SetPatchNo(i) => self.set_patch(i),
            Command::Loop(cmd) => self.loops.interpret(cmd),
            Command::PulseRecord => self.record_beat(),
        }
    }

    fn play_or_arpeggiate(&mut self, command: player::Command) {
        match command {
            NoteOn(_, _, _) | NoteOff(_) =>  {
                if let Some(arp) = &mut self.arpeggiator {
                    arp.interpret(command);
                } else {
                    self.play_transposed(command);
                }
            },
            _ => self.play_transposed(command)
        }
    }

    fn play_transposed(&mut self, command: player::Command) {
        let changed_command = match command {
            NoteOn(pitch, velocity, id) => NoteOn(self.transposer.transpose(pitch), velocity, id),
            other => other,
        };
        self.player.interpret(changed_command)
    }

    fn set_patch(&mut self, i: usize) {
        let patch: Patch = self.patches.get(i).cloned().unwrap_or(Patch::Noop);
        match patch {
            Patch::Instrument(specs) => self.player.interpret(player::Command::SetPatch(specs)),
            Patch::Oscillator(specs) => self.player.set_oscillator(specs),
            Patch::Arpeggiator(seq) => self.set_arpeggiator(seq),
            Patch::Noop => (),
        }
    }

    fn set_arpeggiator(&mut self, seq: Option<Phrase>) {
        self.arpeggiator = seq.map(|s|
            Arpeggiator::new(PitchClass::C, s));
    }

    fn record_beat(&mut self) {
        self.duration_recorder.record();
        if let Some(beat) = self.duration_recorder.read() {
            let pulse_period = Duration::from_millis(beat / PULSES_PER_BEAT);
            self.pulse = self.pulse.with_period(pulse_period);
        }
    }

    fn tick_arpeggiator(&mut self) {
        if let Some(measure_progress) = self.tick_around_measure() {
            let from = self.arp_index;
            let to = self.arp_index + measure_progress;
            self.arp_index = to;
            self.arpeggiator.as_mut()
                .map(|arp| arp.next(from, to)).unwrap_or_else(Vec::default)
                .into_iter().for_each(|cmd| self.play_transposed(cmd));
        }
    }

    fn tick_around_measure(&mut self) -> Option<MeasurePosition> {
        self.pulse.read().map(|PulseReading{ missed, .. }| {
            let pulses_passed = 1 + missed;
            let pulses_per_measure = PULSES_PER_BEAT * BEATS_PER_MEASURE;
            f64::from(pulses_passed) / pulses_per_measure as MeasurePosition
        })
    }

    fn next_sample(&mut self) -> Sample {
        let new_sample = self.player.next_sample();
        let loop_sample = self.loops.next_sample();
        loop_sample + new_sample
    }

}
