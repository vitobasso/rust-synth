use std::sync::mpsc::{Receiver, SyncSender};
use super::{
    Sample, Hz, pulse::Millis, pitch::{Pitch, PitchClass, Semitones},
    instrument::Instrument, oscillator::{self, Oscillator, Specs::Supersaw},
    filter::{BiquadFilter}, modulation::Adsr,
    arpeggiator::Arpeggiator, rhythm::Sequence,
};

const PULSE: Millis = 100;

pub fn run_forever(sample_rate: Hz, patches: Vec<Patch>, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate, patches);
    loop {
        match cmd_in.try_recv() {
            Ok(command) => state.interpret(command),
            _ => (),
        }
        state.arpeggiator.as_mut()
            .and_then(|arp| arp.next())
            .map(|cmd| state.interpret(cmd));

        state.instrument.next_sample().map(|sample|
            signal_out.send(sample).expect("Failed to send a sample")
        );
    }
}

pub enum Command {
    SetPatch(usize),
    NoteOn(Pitch), NoteOff(Pitch), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    Transpose(Semitones),
    ModParam1(f64), ModParam2(f64),
}

#[derive(Clone)]
pub enum Patch {
    Oscillator(oscillator::Specs),
    Arpeggiator(Option<Sequence>),
    Noop,
}

struct State {
    instrument: Instrument,
    arpeggiator: Option<Arpeggiator>,
    patches: Vec<Patch>,
}

impl State {
    pub fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        let instrument = Instrument::new(
            sample_rate,
            Oscillator::new(Supersaw {n_voices: 8, detune_amount: 3.}),
            Box::new(BiquadFilter::lpf(sample_rate)),
            Adsr::new(0.05, 0.2, 0.9, 0.5)
        );
        State { instrument, arpeggiator: None, patches, }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch) => self.handle_note_on(pitch),
            Command::NoteOff(pitch) => self.handle_note_off(pitch),
            Command::ArpNoteOn(pitch) => self.instrument.hold(pitch),
            Command::ArpNoteOff(pitch) => self.instrument.release(pitch),
            Command::Transpose(n) => self.instrument.transpose(n),
            Command::ModParam1(value) => self.instrument.set_mod_1(value),
            Command::ModParam2(value) => self.instrument.set_mod_2(value),
            Command::SetPatch(i) => {
                let patch: Patch = self.patches.get(i).cloned().unwrap_or(Patch::Noop);
                self.set_patch(patch);
            },
        }
    }

    fn handle_note_on(&mut self, pitch: Pitch) {
        match self.arpeggiator.as_mut() {
            Some(arp) =>
                if !arp.is_holding(pitch) {
                    arp.start(pitch)
                },
            None => self.instrument.hold(pitch)
        }
    }

    fn handle_note_off(&mut self, pitch: Pitch) {
        match self.arpeggiator.as_mut() {
            Some(arp) =>
                if arp.is_holding(pitch) {
                    arp.stop();
                    self.instrument.release_any()
                },
            None => self.instrument.release(pitch)
        }
    }

    fn set_patch(&mut self, patch: Patch) {
        match patch {
            Patch::Oscillator(osc) =>
                self.instrument.oscillator = Oscillator::new(osc),
            Patch::Arpeggiator(seq) =>
                self.arpeggiator = seq.map(|s| Arpeggiator::new(PULSE, PitchClass::C, s)),
            Patch::Noop => (),
        }
    }

}
