use crate::core::control::{Millis, playback_controller::Command, duration_recorder::duration_as_millis};
use std::time::Instant;

pub struct Song {
    pub tracks: Vec<Track>
}

pub struct Track {
    events: Vec<ScheduledCommand>,
    pub instrument_id: ChannelId,
}
impl Track {
    pub fn new(events: Vec<ScheduledCommand>, instrument_id: ChannelId) -> Self {
        Track { events, instrument_id }
    }
}

pub type Time = u64;
pub type ScheduledCommand = (Command, Time);
pub type ChannelId = u8;
pub type TargetedCommand = (Command, ChannelId);

pub struct PlayingSong {
    tracks: Vec<PlayingTrack>,
    begin: Instant,
}
impl PlayingSong {

    pub fn new(song: Song) -> Self {
        PlayingSong {
            tracks: song.tracks.into_iter().map(PlayingTrack::new).collect(),
            begin: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Vec<TargetedCommand> {
        let elapsed_time = duration_as_millis(Instant::now() - self.begin);
        self.tracks.iter_mut()
            .flat_map(|t| t.next_targeted(elapsed_time))
            .collect()
    }

}

struct PlayingTrack {
    track: Track,
    current_position: usize,
}
impl PlayingTrack {

    fn new(track: Track) -> Self {
        PlayingTrack { track, current_position: 0 }
    }

    fn next_targeted(&mut self, elapsed_time: Millis) -> Vec<TargetedCommand> {
        self.next(elapsed_time).into_iter()
            .map(|c| (c, self.track.instrument_id))
            .collect()
    }

    fn next(&mut self, elapsed_time: Millis) -> Vec<Command> {
        let begin = self.current_position;
        let result: Vec<Command> = self.track.events.iter()
            .skip(begin).take_while(|(_, t)| *t <= elapsed_time)
            .map(|(cmd, _)| *cmd)
            .collect();
        self.current_position += result.len();
        result
    }

}
