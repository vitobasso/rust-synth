use std::{sync::mpsc::{Receiver, SyncSender}, collections::HashMap};
use core::{
    control::{Millis, arpeggiator::Arpeggiator, loop_recorder::*, duration_recorder::DurationRecorder},
    music_theory::{Hz, Semitones, pitch::{Pitch, PitchClass}, rhythm::Sequence, diatonic_scale::Key},
    synth::{Sample, instrument::{self, Instrument}, oscillator},
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
            .and_then(|arp| arp.next())
            .map(|cmd| state.interpret(cmd));

        let new_sample = state.instrument.next_sample();
        let mix_sample = state.loops.next_sample() + new_sample;
        signal_out.send(mix_sample).expect("Failed to send a sample");

        if let Some(rec) = state.loops.get_recorder() {
            rec.write(new_sample)
        }
    }
}

pub type Id = u32;
pub enum Command {
    NoteOn(Pitch, Id), NoteOff(Pitch, Id), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    ModXY(f64, f64),
    SetPatch(usize),
    LoopPlaybackToggle(usize), LoopRecordingToggle(usize),
    PulseRecord,
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
    patches: Vec<Patch>,
    key: Key, pitch_shift: Semitones,
    holding_notes: HashMap<(Pitch, Id), Pitch>,
    beat: Millis, duration_recorder: DurationRecorder,
    arpeggiator: Option<Arpeggiator>,
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
        State {
            sample_rate, instrument, patches,
            key: PitchClass::C,
            pitch_shift: 0,
            holding_notes: HashMap::new(),
            duration_recorder: DurationRecorder::new(),
            beat: DEFAULT_BEAT,
            arpeggiator: None,
            loops: LoopManager::new(),
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::NoteOn(pitch, id) => self.handle_note_on(pitch, id),
            Command::NoteOff(pitch, id) => self.handle_note_off(pitch, id),
            Command::ArpNoteOn(pitch) => {
                self.instrument.release_all();
                let transposed_pitch = self.transpose(pitch);
                self.instrument.hold(transposed_pitch)
            },
            Command::ArpNoteOff(pitch) => {
                let transposed_pitch = self.transpose(pitch);
                self.instrument.release(transposed_pitch)
            },
            Command::ModXY(x, y) => self.instrument.set_xy_params(x, y),
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
            Command::PulseRecord => self.record_beat(),
        }
    }

    fn handle_note_on(&mut self, input_pitch: Pitch, id: Id) {
        let transposed_pitch = self.transpose(input_pitch);
        if let None = self.holding_notes.insert((input_pitch, id), transposed_pitch) {
            match self.arpeggiator.as_mut() {
                Some(arp) => arp.start(transposed_pitch),
                None => self.instrument.hold(transposed_pitch)
            }
        }
    }

    fn handle_note_off(&mut self, input_pitch: Pitch, id: Id) {
        if let Some(remembered_pitch) = self.holding_notes.remove(&(input_pitch, id)) {
            match self.arpeggiator.as_mut() {
                Some(arp) =>
                    if arp.is_holding(remembered_pitch) {
                        arp.stop();
                        self.instrument.release_all();
                    },
                None => self.instrument.release(remembered_pitch)
            }
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
                self.instrument.set_oscillator(specs),
            Patch::Arpeggiator(seq) =>
                self.arpeggiator = seq.map(|s|
                    Arpeggiator::new(self.get_pulse_millis(), PitchClass::C, s)),
            Patch::Noop => (),
        }
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
