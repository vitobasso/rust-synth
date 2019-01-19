use crate::core::control::{Millis, instrument_player::Command, duration_recorder::duration_as_millis};
use crate::core::music_theory::pitch::PitchClass;
use std::time::Instant;

pub struct Song {
    pub title: String,
    pub key: PitchClass,
    pub voices: Vec<Voice>,
    pub tempo: Vec<Tempo>,
    pub end: Time,
}

pub struct Voice {
    events: Vec<ScheduledCommand>,
    pub instrument_id: ChannelId,
}
impl Voice {
    pub fn new(events: Vec<ScheduledCommand>, instrument_id: ChannelId) -> Self {
        Voice { events, instrument_id }
    }
}

pub type Tempo = u32;
pub type Time = u64;
pub type ScheduledCommand = (Command, Time);
pub type ChannelId = u8;
pub type TargetedCommand = (Command, ChannelId);

pub struct PlayingSong {
    voices: Vec<PlayingVoice>,
    begin: Instant,
}
impl PlayingSong {

    pub fn new(song: Song) -> Self {
        PlayingSong {
            voices: song.voices.into_iter().map(PlayingVoice::new).collect(),
            begin: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Vec<TargetedCommand> {
        let elapsed_time = duration_as_millis(Instant::now() - self.begin);
        self.voices.iter_mut()
            .flat_map(|t| t.next_targeted(elapsed_time))
            .collect()
    }

}

struct PlayingVoice {
    voice: Voice,
    current_position: usize,
}
impl PlayingVoice {

    fn new(voice: Voice) -> Self {
        PlayingVoice { voice, current_position: 0 }
    }

    fn next_targeted(&mut self, elapsed_time: Millis) -> Vec<TargetedCommand> {
        self.next(elapsed_time).into_iter()
            .map(|c| (c, self.voice.instrument_id))
            .collect()
    }

    fn next(&mut self, elapsed_time: Millis) -> Vec<Command> {
        let begin = self.current_position;
        let result: Vec<Command> = self.voice.events.iter()
            .skip(begin).take_while(|(_, t)| *t <= elapsed_time)
            .map(|(cmd, _)| *cmd)
            .collect();
        self.current_position += result.len();
        result
    }

}
