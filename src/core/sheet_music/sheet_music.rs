use crate::core::control::synth::Command;
use crate::core::music_theory::{pitch_class::PitchClass, Modality};
use crate::util;
use std::time::Duration;

pub type Tempo = u32; //microseconds per beat //TODO change to Duration
pub type Tick = u64; //relative time

pub const DEFAULT_TEMPO: Tempo = 500_000; //microseconds per beat
pub const DEFAULT_TICKS_PER_BEAT: u16 = 480;

pub struct SheetMusic {
    pub title: String,
    pub sections: Vec<Section>,
    pub voices: Vec<Voice>,
    pub ticks_per_beat: u16,
    pub end: Tick,
}

impl SheetMusic {

    pub fn count_measures(&self) -> usize {
        let last_section = self.sections.last()
            .unwrap_or_else(|| panic!("Failed to get last section"));
        let last_measure: MeasurePosition = last_section.measure_at_tick(self.end);
        last_measure.floor() as usize + 1
    }

}

impl Default for SheetMusic {
    fn default() -> Self {
        SheetMusic {
            title: String::from("Unnamed"),
            sections: vec!(Section::default()),
            voices: vec![],
            ticks_per_beat: 96,
            end: 0
        }
    }
}

pub type MeasurePosition = f64;

/// Metadata that:
/// - can change over time
/// - is common to all voices
#[derive(PartialEq, Default, Debug)]
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
        *self == Section::default()
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

pub type ScheduledCommand = (Command, Tick); //TODO event with measure position and ref to Section
pub type ChannelId = u8;

pub struct Voice {
    pub events: Vec<ScheduledCommand>,
    pub instrument_id: ChannelId,
}
impl Voice {
    pub fn new(events: Vec<ScheduledCommand>, instrument_id: ChannelId) -> Self {
        Voice { events, instrument_id }
    }
}