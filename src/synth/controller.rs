use std::sync::mpsc::{Receiver, SyncSender};
use super::{
    Sample, Hz, pulse::Millis, pitch::{Pitch, PitchClass, Semitones},
    instrument::{self, Instrument}, oscillator::{self, Oscillator},
    arpeggiator::Arpeggiator, rhythm::Sequence, diatonic_scale::Key,
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
    ShiftPitch(Semitones), ShiftKeyboard(Semitones), TransposeKey(Semitones),
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
    key: Key, pitch_shift: Semitones,
}

impl State {
    pub fn new(sample_rate: Hz, patches: Vec<Patch>) -> State {
        let default_instrument_specs =
            match patches.get(0).cloned().expect("No default patch.") {
                Patch::Instrument(specs) => specs,
                _ => panic!("Default patch must be of type Instrument."),
            };
        let instrument = Instrument::new(default_instrument_specs, sample_rate);
        State { sample_rate, instrument, arpeggiator: None, patches,
                key: PitchClass::C, pitch_shift: 0 }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.handle_note_on(transposed_pitch)
            },
            Command::NoteOff(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.handle_note_off(transposed_pitch)
            },
            Command::ArpNoteOn(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.instrument.hold(transposed_pitch)
            },
            Command::ArpNoteOff(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.instrument.release(transposed_pitch)
            },
            Command::ShiftPitch(n) => self.pitch_shift = self.pitch_shift + n,
            Command::ShiftKeyboard(n) => {
                self.key = self.key + n;
                self.pitch_shift = self.pitch_shift - n;
            }
            Command::TransposeKey(n) => self.key = self.key.circle_of_fifths(n),
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

    fn transpose(&self, pitch: Pitch) -> Pitch {
        let transposed = PitchClass::C.transpose_to(self.key, pitch)
            .expect(&format!("Failed to transpose: {:?}", pitch));
        transposed + self.pitch_shift
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
