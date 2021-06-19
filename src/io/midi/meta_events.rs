use std::{collections::HashMap, mem, time::Duration};
use crate::core::{music_theory::{Modality, pitch_class::*}, sheet_music::sheet_music::*};
use crate::util;
use super::rimd::{MetaCommand, MetaEvent};


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
                minor: *byte2 != 0,
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

pub fn collect_meta_events(events: Vec<ScheduledMeta>, ticks_per_beat: u16) -> SheetMusic {
    let mut music = SheetMusic::default();
    let mut changes_per_section: HashMap<Tick, SectionChanges> = HashMap::default();
    for (meta, tick) in events.into_iter() {
        match meta {
            Meta::TrackName(name) => music.title = name,
            Meta::EndOfTrack => music.end = tick,
            other => {
                let changes = changes_per_section.entry(tick).or_insert_with(SectionChanges::default);
                changes.begin_tick = Some(tick);
                add_section_change(changes, other)
            },
        }
    }

    music.sections = create_sections(changes_per_section, ticks_per_beat);
    music
}

fn add_section_change(section: &mut SectionChanges, event: Meta) {
    match event {
        Meta::KeySignature { sharps, minor } => {
            section.key = Some(PitchClass::C.shift_fifths(sharps));
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
    begin_ticks.sort_unstable();
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
            let time_since_previous = previous.map(|p| {
                let ticks_since_previous = begin_tick - p.begin_tick;
                p.tick_duration.mul_f64(ticks_since_previous as f64)
            });
            let begin_time: Duration = time_since_previous
                .and_then(|time| previous.map(|p| p.begin_time + time))
                .unwrap_or_else(Duration::default);
            let beat_duration = self.beat_duration.or_else(|| previous.map(|p| p.beat_duration))
                .unwrap_or(default.beat_duration);
            let tick_duration = Duration::from_micros(u64::from(beat_duration) / u64::from(ticks_per_beat));
            let beats_per_measure = self.beats_per_measure.or_else(|| previous.map(|p| p.beats_per_measure))
                .unwrap_or(default.beats_per_measure);
            let begin_measure = time_since_previous
                .and_then(|time| previous.map(|p| calculate_section_begin_measure(time, p)))
                .unwrap_or( 0.);
            Section {
                begin_tick, begin_time, begin_measure, beat_duration, tick_duration, beats_per_measure,
                key: self.key.or_else(|| previous.map(|p| p.key)).unwrap_or(default.key),
                modality: self.modality.or_else(|| previous.map(|p| p.modality)).unwrap_or(default.modality),
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

fn calculate_section_begin_measure(time_since_previous_section: Duration, previous_section: &Section) -> f64 {
    let beat_duration = Duration::from_micros(u64::from(previous_section.beat_duration));
    let beats_since_previous = util::duration::div_duration(time_since_previous_section, beat_duration);
    let measures_since_previous = beats_since_previous / f64::from(previous_section.beats_per_measure);
    previous_section.begin_measure + measures_since_previous
}
