use std::sync::mpsc::{Receiver, SyncSender};
use super::{
    Sample, Hz, pulse::Millis, pitch::{Pitch, PitchClass, Semitones},
    instrument::{self, Instrument}, oscillator::{self, Oscillator},
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
    ModXY(f64, f64),
}

#[derive(Clone)]
pub enum Patch {
    Instrument(instrument::Specs),
    Oscillator(oscillator::Specs),
    Arpeggiator(Option<Sequence>),
    Noop,
}

struct State {
    sample_rate: Hz,
    instrument: Instrument,
    arpeggiator: Option<Arpeggiator>,
    patches: Vec<Patch>,
}

impl State {
    pub fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        let default_instrument_specs =
            match patches.get(1).cloned().expect("No default patch.") {
                Patch::Instrument(specs) => specs,
                _ => panic!("Default patch must be of type Instrument."),
            };
        let instrument = Instrument::new(default_instrument_specs, sample_rate);
        State { sample_rate, instrument, arpeggiator: None, patches, }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch) => self.handle_note_on(pitch),
            Command::NoteOff(pitch) => self.handle_note_off(pitch),
            Command::ArpNoteOn(pitch) => self.instrument.hold(pitch),
            Command::ArpNoteOff(pitch) => self.instrument.release(pitch),
            Command::Transpose(n) => self.instrument.transpose(n),
            Command::ModXY(x, y) => self.instrument.set_params(x, y),
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
            Patch::Instrument(specs) =>
                self.instrument = Instrument::new(specs, self.sample_rate),
            Patch::Oscillator(specs) =>
                self.instrument.oscillator = Oscillator::new(specs),
            Patch::Arpeggiator(seq) =>
                self.arpeggiator = seq.map(|s| Arpeggiator::new(PULSE, PitchClass::C, s)),
            Patch::Noop => (),
        }
    }

}
