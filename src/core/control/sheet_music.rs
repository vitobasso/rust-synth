use std::collections::HashMap;
use crate::core::{control::synth, music_theory::Hz, synth::Sample};
use crate::core::sheet_music::{sheet_music::*, playing_music::*};
use crate::io::audio::AudioOut;

///
/// Orchestrates synths to play commands from sheet music
///

pub fn start(music: SheetMusic, sound_out: AudioOut) {
    let mut state = State::new(sound_out.sample_rate(), music);
    sound_out.start(Box::new(|| state.next_sample()));
    loop { //TODO throttle?
        state.tick_music();
    }
}

struct State {
    synths: HashMap<ChannelId, synth::State>,
    music: PlayingMusic,
}

impl State {
    fn new(sample_rate: Hz, sheet_music: SheetMusic) -> State {
        State {
            synths: sheet_music.voices.iter()
                .map(|track| (track.instrument_id, synth::State::with_default_specs(sample_rate)))
                .collect(),
            music: PlayingMusic::new(sheet_music),
        }
    }

    fn interpret(&mut self, (command, channel): TargetedCommand) {
        match self.synths.get_mut(&channel) {
            Some(player) => player.interpret(command),
            None => eprintln!("Player not found for channel: {}", channel),
        }
    }

    fn tick_music(&mut self) {
        self.music.next().commands.into_iter()
            .for_each(|cmd| self.interpret(cmd));
    }

    fn next_sample(&mut self) -> Sample {
        self.synths.values_mut()
            .map(|i| i.next_sample())
            .sum()
    }
}
