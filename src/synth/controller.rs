use std::sync::mpsc::{Receiver, SyncSender};
use std::collections::HashMap;
use super::{
    Sample, Millis, Hz, pitch::{Pitch, PitchClass, Semitones},
    instrument::{self, Instrument}, oscillator,
    arpeggiator::Arpeggiator, rhythm::Sequence, diatonic_scale::Key,
    loop_recorder::*,
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

        let sample = state.instrument.next_sample();

        let mix = state.loops.next_sample() + sample;

        signal_out.send(mix).expect("Failed to send a sample");

        if let Some(rec) = state.loops.get_recorder() {
            rec.write(sample)
        }
    }
}

pub enum Command {
    NoteOn(Pitch), NoteOff(Pitch), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    ModXY(f64, f64),
    SetPatch(usize),
    LoopPlaybackToggle(usize), LoopRecordingToggle(usize),
    ShiftPitch(Semitones), ShiftKeyboard(Semitones), TransposeKey(Semitones),
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
    holding_notes: HashMap<Pitch, Pitch>,
    loops: LoopManager,
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
                key: PitchClass::C, pitch_shift: 0, holding_notes: HashMap::new(),
                loops: LoopManager::new() }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch) => self.handle_note_on(pitch),
            Command::NoteOff(pitch) => self.handle_note_off(pitch),
            Command::ArpNoteOn(pitch) => {
                self.instrument.release_all();
                let transposed_pitch = self.transpose(pitch);
                self.instrument.hold(transposed_pitch)
            },
            Command::ArpNoteOff(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.instrument.release(transposed_pitch)
            },
            Command::ModXY(x, y) => self.instrument.set_params(x, y),
            Command::ShiftPitch(n) => self.pitch_shift = self.pitch_shift + n,
            Command::ShiftKeyboard(n) => {
                self.key = self.key + n;
                self.pitch_shift = self.pitch_shift - n;
            }
            Command::TransposeKey(n) => self.key = self.key.circle_of_fifths(n),
            Command::SetPatch(i) => {
                let patch: Patch = self.patches.get(i).cloned().unwrap_or(Patch::Noop);
                self.set_patch(patch);
            },
            Command::LoopRecordingToggle(i) => self.loops.toggle_recording(i),
            Command::LoopPlaybackToggle(i) => self.loops.toggle_playback(i),
        }
    }

    fn handle_note_on(&mut self, pitch: Pitch) {
        let transposed_pitch = self.transpose(pitch);
        self.holding_notes.insert(pitch, transposed_pitch);
        match self.arpeggiator.as_mut() {
            Some(arp) =>
                if !arp.is_holding(transposed_pitch) {
                    arp.start(transposed_pitch)
                },
            None => self.instrument.hold(transposed_pitch)
        };
    }

    fn handle_note_off(&mut self, pitch: Pitch) {
        self.holding_notes.remove(&pitch).map(|remembered_pitch|
            match self.arpeggiator.as_mut() {
                Some(arp) =>
                    if arp.is_holding(remembered_pitch) {
                        arp.stop();
                        self.instrument.release_all();
                    },
                None => self.instrument.release(remembered_pitch)
            }
        );
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
                self.instrument.set_oscillator(specs),
            Patch::Arpeggiator(seq) =>
                self.arpeggiator = seq.map(|s| Arpeggiator::new(PULSE, PitchClass::C, s)),
            Patch::Noop => (),
        }
    }

}
