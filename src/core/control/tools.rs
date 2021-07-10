use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;
use crate::core::{
    control::{synth::{self, Command::*}},
    music_theory::{Hz, pitch_class::PitchClass},
    synth::{instrument, Sample},
    tools::{pulse, transposer, loops, arpeggiator, arpeggiator::phrase::Phrase, tap_tempo, Millis},
    sheet_music::sheet_music::MeasurePosition,
};

///
/// Connects tools and synth together, interprets commands and delegates to them
///

pub fn start(sample_rate: Hz, command_in: Receiver<Command>, sound_out: SyncSender<Sample>, view_out: SyncSender<View>) {
    let command_rate = 10; //TODO in hz
    let view_refresh_rate = 1000; //TODO in hz
    let mut state = State::new(sample_rate);
    for i in 0.. {
        if i % command_rate == 0 {
            if let Ok(command) = command_in.try_recv() {
                state.interpret(command);
            }
            state.tick_arpeggiator();
        }

        let new_sample = state.next_sample();
        sound_out.send(new_sample).expect("Failed to send a sample");
        state.loops.write(new_sample);

        if i % view_refresh_rate == 0 {
            let view = state.view();
            let _ = view_out.try_send(view);
        }
    }
}

const BEATS_PER_MEASURE: u64 = 4;
const PULSES_PER_BEAT: u64 = 32;
const DEFAULT_PULSE: Millis = 12;

#[derive(Clone)]
pub enum Command {
    Instrument(synth::Command),
    Transposer(transposer::Command),
    SetPatch(Patch),
    Loop(loops::Command),
    TapTempo,
}

#[derive(Clone)]
pub enum Patch {
    Instrument(instrument::Specs),
    ArpeggiatorPhrase(Option<Phrase>), //TODO deprecate
    Arpeggiator(Option<arpeggiator::Specs>),
    Noop,
}

pub struct State {
    synth: synth::State,
    transposer: transposer::State,
    pulse: pulse::Pulse,
    arpeggiator: Option<arpeggiator::Arpeggiator>,
    arp_index: f64,
    tap_tempo: tap_tempo::TapTempo,
    loops: loops::Manager,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub synth: synth::View,
    pub transposer: transposer::State,
    pub pulse: pulse::View,
    pub arpeggiator: Option<arpeggiator::View>,
    pub arp_index: f64,
    pub tap_tempo: tap_tempo::TapTempo,
    pub loops: loops::View,
}

impl State {
    fn new(sample_rate: Hz) -> State {
        State {
            synth: synth::State::new(sample_rate),
            transposer: transposer::State::new(PitchClass::C),
            tap_tempo: Default::default(),
            pulse: pulse::Pulse::new_with_millis(DEFAULT_PULSE),
            arpeggiator: None,
            arp_index: 0.,
            loops: Default::default(),
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(cmd) => self.play_or_arpeggiate(cmd),
            Command::Transposer(cmd) => self.transposer.interpret(cmd),
            Command::SetPatch(patch) => self.set_patch(patch),
            Command::Loop(cmd) => self.loops.interpret(cmd),
            Command::TapTempo => self.tap_tempo(),
        }
    }

    fn play_or_arpeggiate(&mut self, command: synth::Command) {
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

    fn play_transposed(&mut self, command: synth::Command) {
        let changed_command = match command {
            NoteOn(pitch, velocity, id) => NoteOn(self.transposer.transpose(pitch), velocity, id),
            other => other,
        };
        self.synth.interpret(changed_command)
    }

    fn set_patch(&mut self, patch: Patch) {
        match patch {
            Patch::Instrument(specs) => self.synth.interpret(SetPatch(specs)),
            Patch::Arpeggiator(specs) => self.set_arpeggiator(specs),
            Patch::ArpeggiatorPhrase(seq) => self.set_arpeggiator_phrase(seq),
            Patch::Noop => (),
        }
    }

    fn set_arpeggiator(&mut self, specs: Option<arpeggiator::Specs>) {
        let old_state = self.arpeggiator.as_ref().map(|a| a.state());
        self.arpeggiator = specs.map(|s| {
            let mut arp = arpeggiator::Arpeggiator::from_specs(s);
            if let Some(state) = old_state {
                arp.set_state(state);
            }
            arp
        });
    }

    fn set_arpeggiator_phrase(&mut self, phrase: Option<Phrase>) {
        let old_state = self.arpeggiator.as_ref().map(|a| a.state());
        self.arpeggiator = phrase.map(|p| {
            let mut arp = arpeggiator::Arpeggiator::from_phrase(PitchClass::C, p);
            if let Some(state) = old_state {
                arp.set_state(state);
            }
            arp
        });
    }

    fn tap_tempo(&mut self) {
        self.tap_tempo.tap();
        if let Some(beat) = self.tap_tempo.read() {
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
        self.pulse.read().map(|pulse::PulseReading{ missed, .. }| {
            let pulses_passed = 1 + missed;
            let pulses_per_measure = PULSES_PER_BEAT * BEATS_PER_MEASURE;
            f64::from(pulses_passed) / pulses_per_measure as MeasurePosition
        })
    }

    fn next_sample(&mut self) -> Sample {
        let new_sample = self.synth.next_sample();
        let loop_sample = self.loops.next_sample();
        loop_sample + new_sample
    }

    pub fn view(&self) -> View {
        View {
            synth: self.synth.view(),
            transposer: self.transposer.clone(),
            pulse: self.pulse.view(),
            arpeggiator: self.arpeggiator.as_ref().map(|a| a.view()),
            arp_index: self.arp_index,
            tap_tempo: self.tap_tempo.clone(),
            loops: self.loops.view(),
        }
    }

}
