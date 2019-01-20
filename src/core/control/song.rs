use crate::core::control::{Millis, instrument_player::Command, duration_recorder::duration_as_millis};
use crate::core::music_theory::pitch::PitchClass;
use std::time::Instant;
use std::time::Duration;

pub const DEFAULT_TEMPO: Tempo = 500_000; //microseconds per beat
pub const DEFAULT_TICKS_PER_BEAT: u16 = 480;

pub struct Song {
    pub title: String,
    pub key: PitchClass,
    pub voices: Vec<Voice>,
    pub tempo: Tempo,
    pub end: Tick,
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

pub type Tempo = u32; //microseconds per beat
pub type Tick = u64; //relative time
pub type ScheduledCommand = (Command, Tick);
pub type ChannelId = u8;
pub type TargetedCommand = (Command, ChannelId);

pub struct PlayingSong {
    voices: Vec<PlayingVoice>,
    tempo: Tempo,
    begin: Instant,
    end: Instant,
}
impl PlayingSong {

    pub fn new(song: Song) -> Self {
        let begin = Instant::now();
        let song_duration = Duration::from_millis(to_absolute_time(song.end, song.tempo));
        PlayingSong {
            voices: song.voices.into_iter().map(PlayingVoice::new).collect(),
            tempo: song.tempo,
            begin,
            end: begin + song_duration,
        }
    }

    pub fn next(&mut self) -> Vec<TargetedCommand> {
        let elapsed_time = duration_as_millis(Instant::now() - self.begin);
        let tempo = self.tempo;
        self.voices.iter_mut()
            .flat_map(|t| t.next_targeted(elapsed_time, tempo))
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

    fn next_targeted(&mut self, elapsed_time: Millis, tempo: Tempo) -> Vec<TargetedCommand> {
        self.next(elapsed_time, tempo).into_iter()
            .map(|c| (c, self.voice.instrument_id))
            .collect()
    }

    fn next(&mut self, elapsed_time: Millis, tempo: Tempo) -> Vec<Command> {
        let begin = self.current_position;
        let result: Vec<Command> = self.voice.events.iter()
            .skip(begin).take_while(|(_, t)| {
                to_absolute_time(*t, tempo) <= elapsed_time
            })
            .map(|(cmd, _)| *cmd)
            .collect();
        self.current_position += result.len();
        result
    }

}

fn to_absolute_time(relative: Tick, tempo: Tempo) -> Tick {
    let ratio = tempo as f64 / DEFAULT_TEMPO as f64;
    (relative as f64 * ratio) as u64
}