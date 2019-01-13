use std::sync::mpsc::SyncSender;
use crate::core::{
    control::{instrument_player::{self as player}, song::*},
    music_theory::Hz, synth::Sample,
    synth::instrument::Specs,
};

pub fn loop_forever(sample_rate: Hz, presets: Vec<Specs>, song_specs: Song, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate, presets, song_specs);
    loop {
        let players = &mut state.players;
        let song = &mut state.song;
        song.next().iter()
            .for_each(|(cmd, instrument_id)| {
                let cyclic_instrument_id: usize = *instrument_id as usize % players.len();
                players[cyclic_instrument_id].interpret(*cmd)
            });

        let mix_sample = players.iter_mut()
            .map(|i| i.next_sample())
            .sum();
        signal_out.send(mix_sample).expect("Failed to send a sample");
    }
}

struct State {
    players: Vec<player::State>,
    song: PlayingSong,
}

impl State {
    fn new(sample_rate: Hz, patches: Vec<Specs>, song_specs: Song) -> State {
        let players = patches.into_iter()
            .map(|patch| player::State::new(patch, sample_rate))
            .collect();
        State {
            players,
            song: PlayingSong::new(song_specs),
        }
    }
}
