use crate::core::control::instrument_player::Command;
use crate::core::music_theory::{pitch::PitchClass, Modality};
use crate::util;
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
#[derive(Debug)]
pub struct Section {
    pub begin_tick: Tick,
    pub begin_time: Duration,
    pub begin_measure: MeasurePosition,
    pub key: PitchClass,
    pub modality: Modality,
    pub beat_duration: Tempo,
    pub beats_per_measure: u8,
    pub tick_duration: Duration,
}
impl Section {
    pub fn is_default(&self) -> bool {
        self.begin_tick == 0 &&
        self.begin_time == Duration::default() &&
        self.begin_measure == 0. &&
        self.key == PitchClass::C &&
        self.modality == Modality::MAJOR &&
        self.beat_duration == DEFAULT_TEMPO &&
        self.beats_per_measure == 4 &&
        self.tick_duration == Duration::default()
    }

    pub fn measure_at_time(&self, time_elapsed: Duration) -> MeasurePosition {
        let time_in_section = time_elapsed - self.begin_time; //FIXME panic if out of section
        self.measure_at(time_in_section)
    }

    pub fn measure_at_tick(&self, tick: Tick) -> MeasurePosition {
        let tick_in_section = tick - self.begin_tick; //FIXME panic if out of section
        let time_in_section = self.tick_duration.mul_f64(tick_in_section as f64);
        self.measure_at(time_in_section)
    }

    fn measure_at(&self, time_in_section: Duration) -> MeasurePosition {
        let beat_duration = Duration::from_micros(u64::from(self.beat_duration));
        let measure_duration = beat_duration * u32::from(self.beats_per_measure);
        let measures_in_section = util::duration::div_duration(time_in_section, measure_duration);
        self.begin_measure + measures_in_section
    }
}
impl Default for Section {
    fn default() -> Self {
        Section {
            begin_tick: 0,
            begin_time: Duration::default(),
            begin_measure: 0.,
            key: PitchClass::C,
            modality: Modality::MAJOR,
            beat_duration: DEFAULT_TEMPO,
            beats_per_measure: 4,
            tick_duration: Duration::default(),
        }
    }
}

pub type Tempo = u32; //microseconds per beat //TODO change to Duration
pub type Tick = u64; //relative time
pub type ScheduledCommand = (Command, Tick);
pub type ChannelId = u8;
pub type TargetedCommand = (Command, ChannelId);
pub type MeasurePosition = f64;

pub struct PlayingSong {
    sections: Vec<Section>,
    voices: Vec<PlayingVoice>,
    begin: Instant,
    #[allow(dead_code)] end: Instant,
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

    pub fn next(&mut self) -> SongReading {
        let time_elapsed = Instant::now() - self.begin;
        self.update_current_section_index(time_elapsed);
        let section = self.sections.get(self.current_section_index)
            .unwrap_or_else(|| panic!("No current section"));
        let commands =self.voices.iter_mut()
            .flat_map(|t| t.next_targeted(time_elapsed, section))
            .collect();
        SongReading { commands, time_elapsed, section }
    }

    fn update_current_section_index(&mut self, elapsed_time: Duration) {
        self.current_section_index = (self.current_section_index..self.sections.len())
            .take_while(|i| self.sections.get(*i).map(|section| section.begin_time <= elapsed_time) == Some(true))
            .last().unwrap_or_else(|| panic!("Failed to find section"));
    }

    pub fn tick_duration(&self) -> Duration {
        self.sections.get(self.current_section_index)
            .unwrap_or_else(|| panic!("No current section"))
            .tick_duration
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
    let nanos = ticks as u64 * section.tick_duration.as_nanos() as u64;
    section.begin_time + Duration::from_nanos(nanos)
}

fn song_duration(song: &Song) -> Duration {
    let default_section = Section::default();
    let last_section = song.sections.last().unwrap_or(&default_section);
    get_time(song.end, last_section)
}

pub struct SongReading<'a> {
    pub commands: Vec<TargetedCommand>,
    pub time_elapsed: Duration,
    pub section: &'a Section,
}
impl <'a> SongReading<'a>  {
    pub fn measure(&self) -> MeasurePosition {
        self.section.measure_at_time(self.time_elapsed)
    }
}