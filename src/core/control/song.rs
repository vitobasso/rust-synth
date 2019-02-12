use crate::core::control::instrument_player::Command;
use crate::core::music_theory::{pitch::PitchClass, Modality};
use std::time::{Instant, Duration};

pub const DEFAULT_TEMPO: Tempo = 500_000; //microseconds per beat
pub const DEFAULT_TICKS_PER_BEAT: u16 = 480;

pub struct Song {
    pub title: String,
    pub sections: Vec<Section>,
    pub voices: Vec<Voice>,
    pub ticks_per_beat: u16,
    pub end: Tick,
}

pub struct Voice {
    pub events: Vec<ScheduledCommand>,
    pub instrument_id: ChannelId,
}
impl Voice {
    pub fn new(events: Vec<ScheduledCommand>, instrument_id: ChannelId) -> Self {
        Voice { events, instrument_id }
    }
}

/// Metadata that:
/// - can change over time
/// - is common to all voices
#[derive(PartialEq, Eq, Debug)]
pub struct Section {
    pub begin_tick: Tick,
    pub begin_time: Duration,
    pub key: PitchClass,
    pub modality: Modality,
    pub beat_duration: Tempo,
    pub beats_per_measure: u8,
    pub tick_duration: Duration,
}
impl Default for Section {
    fn default() -> Self {
        Section {
            begin_tick: 0,
            begin_time: Duration::default(),
            key: PitchClass::C,
            modality: Modality::MAJOR,
            beat_duration: DEFAULT_TEMPO,
            beats_per_measure: 4,
            tick_duration: Duration::default(),
        }
    }
}

pub type Tempo = u32; //microseconds per beat
pub type Tick = u64; //relative time
pub type ScheduledCommand = (Command, Tick);
pub type ChannelId = u8;
pub type TargetedCommand = (Command, ChannelId);

impl Default for Song {
    fn default() -> Self {
        Song {
            title: String::from("Unnamed"),
            sections: vec!(Section::default()),
            voices: vec![],
            ticks_per_beat: 96,
            end: 0
        }
    }
}

pub struct PlayingSong {
    sections: Vec<Section>,
    voices: Vec<PlayingVoice>,
    begin: Instant,
    end: Instant,
    current_section_index: usize,
}
impl PlayingSong {

    pub fn new(song: Song) -> Self {
        let begin = Instant::now();
        PlayingSong {
            begin,
            end: begin + song_duration(&song),
            sections: song.sections,
            voices: song.voices.into_iter().map(PlayingVoice::new).collect(),
            current_section_index: 0,
        }
    }

    pub fn next(&mut self) -> Vec<TargetedCommand> {
        let elapsed_time = Instant::now() - self.begin;
        self.update_current_section_index(elapsed_time);
        let section = self.sections.get(self.current_section_index)
            .unwrap_or_else(|| panic!("No current section"));
        self.voices.iter_mut()
            .flat_map(|t| t.next_targeted(elapsed_time, section))
            .collect()
    }

    fn update_current_section_index(&mut self, elapsed_time: Duration) {
        self.current_section_index = (self.current_section_index..self.sections.len())
            .take_while(|i| self.sections.get(*i).map(|section| section.begin_time < elapsed_time) == Some(true))
            .last().unwrap_or_else(|| panic!("No sections"));
    }

}

struct PlayingVoice {
    voice: Voice,
    current_event_index: usize,
}
impl PlayingVoice {

    fn new(voice: Voice) -> Self {
        PlayingVoice { voice, current_event_index: 0 }
    }

    fn next_targeted(&mut self, elapsed_time: Duration, section: &Section) -> Vec<TargetedCommand> {
        self.next(elapsed_time, section).into_iter()
            .map(|c| (c, self.voice.instrument_id))
            .collect()
    }

    fn next(&mut self, elapsed_time: Duration, section: &Section) -> Vec<Command> {
        let begin = self.current_event_index;
        let result: Vec<Command> = self.voice.events.iter()
            .skip(begin)
            .take_while(|(_, t)| get_time(*t, section) <= elapsed_time)
            .map(|(cmd, _)| *cmd)
            .collect();
        self.current_event_index += result.len();
        result
    }

}

fn get_time(tick: Tick, section: &Section) -> Duration {
    let ticks = tick - section.begin_tick;
    let nanos = ticks as u64 * duration_as_nanos(section.tick_duration);
    section.begin_time + Duration::from_nanos(nanos)
}

fn song_duration(song: &Song) -> Duration {
    let default_section = Section::default();
    let last_section = song.sections.last().unwrap_or_else(|| &default_section);
    get_time(song.end, last_section)
}

//TODO replace with Duration.as_nanos after rust 1.33
fn duration_as_nanos(duration: Duration) -> u64 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    secs * 1_000_000 + u64::from(nanos)
}