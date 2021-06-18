use std::sync::mpsc::SyncSender;
use std::collections::HashMap;
use crate::core::{
    control::{instrument_player as player, song::*}, music_theory::Hz, synth::Sample,
};

pub fn start(sample_rate: Hz, song_specs: Song, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate, song_specs);
    loop {
        state.tick_song();
        let sample = state.next_sample();
        signal_out.send(sample).expect("Failed to send a sample");
    }
}

struct State {
    players: HashMap<ChannelId, player::State>,
    song: PlayingSong,
}

impl State {
    fn new(sample_rate: Hz, song_specs: Song) -> State {
        State {
            players: song_specs.voices.iter()
                .map(|track| (track.instrument_id, player::State::with_default_specs(sample_rate)))
                .collect(),
            song: PlayingSong::new(song_specs),
        }
    }

    fn interpret(&mut self, (command, channel): TargetedCommand) {
        match self.players.get_mut(&channel) {
            Some(player) => player.interpret(command),
            None => eprintln!("Player not found for channel: {}", channel),
        }
    }

    fn tick_song(&mut self) {
        self.song.next().commands.into_iter()
            .for_each(|cmd| self.interpret(cmd));
    }

    fn next_sample(&mut self) -> Sample {
        self.players.values_mut()
            .map(|i| i.next_sample())
            .sum()
    }
}
