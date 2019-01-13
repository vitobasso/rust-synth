use crate::core::music_theory::{Semitones, pitch::Pitch, diatonic_scale::Key};

#[derive(Clone, Copy, Debug)]
pub enum Command {
    TransposeKey(Semitones),
    ShiftPitch(Semitones),
    ShiftKeyboard(Semitones),
}

pub struct State {
    input_key: Key,
    transposed_key: Key,
    pitch_shift: Semitones,
}

impl State {

    pub fn new(key: Key) -> State {
        State {
            input_key: key,
            transposed_key: key,
            pitch_shift: 0,
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::TransposeKey(n) => self.transposed_key = self.transposed_key.circle_of_fifths(n),
            Command::ShiftPitch(n) => self.pitch_shift += n,
            Command::ShiftKeyboard(n) => {
                self.transposed_key += n;
                self.pitch_shift -= n;
            }
        }
    }

    pub fn transpose(&self, pitch: Pitch) -> Pitch {
        let transposed = self.input_key.transpose_to(self.transposed_key, pitch)
            .unwrap_or_else(|| panic!("Failed to transpose: {:?}", pitch));
        transposed + self.pitch_shift
    }

}