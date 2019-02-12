use super::rimd::{MetaEvent, MetaCommand};
use crate::core::control::song::Tempo;
use std::{mem, collections::HashMap, time::Duration};
use crate::core::{ control::{ song::* }, music_theory::{pitch::*, Modality} };

#[derive(Debug)]
pub enum Meta {
    TrackName(String),
    InstrumentName(String),
    TimeSignature {
        numerator: u8, //beats per measure
        denominator: u8, //quarter notes per beat
        metronome_period: u8, //MIDI ticks per metronome quarter. Default: 24
        rate_32ths: u8 //32th notes per MIDI quarter note. Default: 8
    },
    KeySignature { sharps: i8, minor: bool },
    TempoSetting(Tempo), // microseconds per quarter note
    EndOfTrack,
}

pub type ScheduledMeta = (Meta, Tick);

pub fn decode_meta_event(msg: &MetaEvent) -> Option<Meta> {
    match msg.command {
        MetaCommand::SequenceOrTrackName =>
            String::from_utf8(msg.data.clone()).map(Meta::TrackName).ok(),
        MetaCommand::InstrumentName =>
            String::from_utf8(msg.data.clone()).map(Meta::InstrumentName).ok(),
        MetaCommand::EndOfTrack => Some(Meta::EndOfTrack),
        MetaCommand::TempoSetting => decode_tempo_setting(msg.data.as_slice()),
        MetaCommand::TimeSignature => decode_time_signature(msg.data.as_slice()),
        MetaCommand::KeySignature => decode_key_signature(msg.data.as_slice()),
        other => {
            println!("MIDI: Ignored meta event: {:?}", other);
            None
        }
    }
}

fn decode_tempo_setting(data: &[u8]) -> Option<Meta> {
    match data {
        [byte1, byte2, byte3] => {
            let array: [u8; 4] = [*byte3, *byte2, *byte1, 0];
            let microsecs_per_quarternote: u32 = unsafe { mem::transmute(array) };
            Some(Meta::TempoSetting(microsecs_per_quarternote))
        }
        _ => {
            eprintln!("MIDI: Invalid meta event: TempoSignature, data={:?}", data);
            None
        }
    }
}

fn decode_key_signature(data: &[u8]) -> Option<Meta> {
    match data {
        [byte1, byte2] => {
            let meta = Meta::KeySignature {
                sharps: unsafe { mem::transmute(*byte1) },
                minor: unsafe { mem::transmute(*byte2) },
            };
            Some(meta)
        }
        _ => {
            eprintln!("MIDI: Invalid meta event: KeySignature, data={:?}", data);
            None
        }
    }
}

/// http://www.deluge.co/?q=midi-tempo-bpm
fn decode_time_signature(data: &[u8]) -> Option<Meta> {
    match data {
        [byte1, byte2, byte3, byte4] => {
            let denom_power: u8 = unsafe { mem::transmute(*byte2) };
            let meta = Meta::TimeSignature {
                numerator: unsafe { mem::transmute(*byte1) },
                denominator: 2_u8.pow(denom_power as u32),
                metronome_period: unsafe { mem::transmute(*byte3) },
                rate_32ths: unsafe { mem::transmute(*byte4) },
            };
            Some(meta)
        }
        _ => {
            eprintln!("MIDI: Invalid meta event: TimeSignature, data={:?}", data);
            None
        }
    }
}

pub fn collect_meta_events(events: Vec<ScheduledMeta>, ticks_per_beat: u16) -> Song {
    let mut song = Song::default();
    let mut changes_per_section: HashMap<Tick, SectionChanges> = HashMap::default();
    for (meta, tick) in events.into_iter() {
        match meta {
            Meta::TrackName(name) => song.title = name,
            Meta::EndOfTrack => song.end = tick,
            other => {
                let changes = changes_per_section.entry(tick).or_insert_with(|| SectionChanges::default());
                changes.begin_tick = Some(tick);
                add_section_change(changes, other)
            },
        }
    }

    song.sections = create_sections(changes_per_section, ticks_per_beat);
    song
}

fn add_section_change(section: &mut SectionChanges, event: Meta) {
    match event {
        Meta::KeySignature { sharps, minor } => {
            section.key = Some(PitchClass::C.circle_of_fifths(sharps));
            section.modality = Some(if minor {Modality::MINOR} else {Modality::MAJOR});
        },
        Meta::TempoSetting(t) =>
            section.beat_duration = Some(t),
        Meta::TimeSignature { numerator: n, .. } =>
            section.beats_per_measure = Some(n),
        _ => (),
    }
}

fn create_sections(changes_per_section: HashMap<Tick, SectionChanges>, ticks_per_beat: u16) -> Vec<Section> {
    let mut result = vec!();
    let mut begin_ticks: Vec<Tick> = changes_per_section.keys().cloned().collect::<Vec<_>>();
    begin_ticks.sort();
    for i in 0..changes_per_section.len() {
        let previous = result.last();
        let current = begin_ticks.get(i)
            .and_then(|tick| changes_per_section.get(tick))
            .and_then(|changes| changes.to_section(previous, ticks_per_beat));
        if let Some(section) = current {
            result.push(section)
        }
    }
    result
}

/// Incremental changes on top of the previous Section
#[derive(PartialEq, Eq)]
struct SectionChanges {
    begin_tick: Option<Tick>,
    begin_time: Option<Duration>,
    key: Option<PitchClass>,
    modality: Option<Modality>,
    beat_duration: Option<Tempo>,
    beats_per_measure: Option<u8>,
}
impl SectionChanges {
    fn to_section(&self, previous: Option<&Section>, ticks_per_beat: u16) -> Option<Section> {
        let default = Section::default();
        self.begin_tick.map(|begin_tick| {
            let begin_time: Duration = previous.map(|p| calculate_section_begin_time(begin_tick, p))
                .unwrap_or_else(|| Duration::default());
            let beat_duration = self.beat_duration.or(previous.map(|p| p.beat_duration))
                .unwrap_or_else(|| default.beat_duration);
            let tick_duration = Duration::from_micros(beat_duration as u64 / ticks_per_beat as u64);
            Section {
                begin_tick, begin_time, beat_duration, tick_duration,
                key: self.key.or(previous.map(|p| p.key)).unwrap_or_else(|| default.key),
                modality: self.modality.or(previous.map(|p| p.modality)).unwrap_or_else(|| default.modality),
                beats_per_measure: self.beats_per_measure.or(previous.map(|p| p.beats_per_measure))
                    .unwrap_or_else(|| default.beats_per_measure),
            }
        })
    }
}
impl Default for SectionChanges {
    fn default() -> Self {
        SectionChanges {
            begin_tick: None,
            begin_time: None,
            key: None,
            modality: None,
            beat_duration: None,
            beats_per_measure: None,
        }
    }
}

fn calculate_section_begin_time(begin_tick: Tick, previous_section: &Section) -> Duration {
    let ticks = begin_tick - previous_section.begin_tick;
    let millis = previous_section.beat_duration as u64 * ticks;
    Duration::from_millis(millis)
}
