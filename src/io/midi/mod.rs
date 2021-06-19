use std::{collections::HashMap, path::Path};

use rimd;

use crate::core::{
    control::{synth::{Command, Command::*, id}},
    music_theory::pitch::*,
};
use crate::core::sheet_music::sheet_music::*;

use self::meta_events::{collect_meta_events, decode_meta_event, ScheduledMeta};
use self::rimd::{Event as RimdEvent, MidiMessage, SMF, SMFError,
                 Status, Track as RimdTrack, TrackEvent as RimdTrackEvent};

mod patch;
mod meta_events;

pub fn read_file(file_path: &str) -> Option<SheetMusic> {
    println!("MIDI: Reading file: {}", file_path);
    match SMF::from_file(Path::new(file_path)) {
        Ok(smf) =>
            Some(decode_midi_file(&smf))
        ,
        Err(e) => {
            match e {
                SMFError::InvalidSMFFile(s) => println!("Invalid Midi file: {}", s),
                SMFError::Error(e) => println!("IO Error: {}", e),
                SMFError::MidiError(e) => println!("Midi Error: {}", e),
                SMFError::MetaError(_) => println!("Meta Error"),
            };
            None
        }
    }
}

fn decode_midi_file(midi_file: &SMF) -> SheetMusic {
    assert!(midi_file.division > 0, "MIDI: Unsupported format. Header has negative division.");
    let ticks_per_beat: u16 = midi_file.division as u16;
    let music = SheetMusic { ticks_per_beat, ..Default::default() };
    midi_file.tracks.iter()
        .map(|track| decode_track(track, ticks_per_beat))
        .fold(music, merge_tracks)
}

fn merge_tracks(mut left: SheetMusic, mut right: SheetMusic) -> SheetMusic {
    let mut left_voices = std::mem::take(&mut left.voices);
    let mut right_voices = std::mem::take(&mut right.voices);
    left_voices.append(&mut right_voices);
    let default = SheetMusic::default();
    SheetMusic {
        title: if left.title != default.title {left.title} else {right.title},
        sections: if !is_default_sections(&left.sections) {left.sections} else {right.sections},
        ticks_per_beat: if left.ticks_per_beat != default.ticks_per_beat {left.ticks_per_beat} else {right.ticks_per_beat},
        end: if left.end > right.end {left.end} else {right.end},
        voices: left_voices,
    }
}
fn is_default_sections(sections: &[Section]) -> bool {
    match sections {
        [section] => section.is_default(),
        _ => false
    }
}

fn decode_track(track: &RimdTrack, ticks_per_beat: u16) -> SheetMusic {
    let mixed_events: Vec<Event> = decode_events(track);
    let (commands_by_channel, meta_events) = organize_events(mixed_events);

    let mut music = collect_meta_events(meta_events, ticks_per_beat);
    music.voices = collect_note_events(commands_by_channel);
    music
}

fn collect_note_events(commands_by_channel: HashMap<ChannelId, Vec<ScheduledCommand>>) -> Vec<Voice> {
    commands_by_channel.into_iter()
        .map(|(channel, events)| Voice::new(events, channel))
        .collect()
}

fn organize_events(events: Vec<Event>) -> (HashMap<ChannelId, Vec<ScheduledCommand>>, Vec<ScheduledMeta>) {
    let mut commands_by_channel: HashMap<ChannelId, Vec<ScheduledCommand>> = HashMap::default();
    let mut meta_events: Vec<ScheduledMeta> = Vec::default();
    for event in events.into_iter() {
        match event {
            Event::Midi(cmd, channel) =>
                commands_by_channel.entry(channel).or_insert_with(Vec::default).push(cmd),
            Event::Meta(meta) =>
                meta_events.push(meta),
        }
    }
    (commands_by_channel, meta_events)
}

fn decode_events(track: &RimdTrack) -> Vec<Event> {
    let events = track.events.iter()
        .filter_map(decode_event)
        .collect();
    accumulate_time(events)
}

fn accumulate_time(events: Vec<Event>) -> Vec<Event> {
    events.into_iter()
        .scan(0, |accumulated_time, event| match event {
            Event::Midi((cmd, time), channel) => {
                *accumulated_time += time;
                Some(Event::Midi((cmd, *accumulated_time), channel))
            },
            Event::Meta((cmd, time)) => {
                *accumulated_time += time;
                Some(Event::Meta((cmd, *accumulated_time)))
            }
        }).collect()
}

enum Event {
    Midi(ScheduledCommand, ChannelId),
    Meta(ScheduledMeta)
}

fn decode_event(event: &RimdTrackEvent) -> Option<Event> {
    match event.event {
        RimdEvent::Midi(ref message) =>
            message.channel().and_then(|channel|
                decode_note_event(message)
                    .map(|cmd| ((cmd, event.vtime), channel))
            ).map(|(cmd, channel)|Event::Midi(cmd, channel)),
        RimdEvent::Meta(ref meta) => {
            decode_meta_event(meta).map(|meta| Event::Meta((meta, event.vtime)))
        },

    }
}

fn decode_note_event(msg: &MidiMessage) -> Option<Command> {
    match msg.data.as_slice() {
        [_, pitch_byte, velocity_byte] => {
            let pitch = Pitch::from_index(*pitch_byte as usize);
            let velocity: f64 = *velocity_byte as f64 / u8::MAX as f64;
            let note_on = NoteOn(pitch, velocity, id(pitch));
            let note_off = NoteOff(id(pitch));
            match (msg.status(), *velocity_byte) {
                (Status::NoteOn, 0) => Some(note_off),
                (Status::NoteOn, _) => Some(note_on),
                (Status::NoteOff, _) => Some(note_off),
                _ => None,
            }
        }
        [_, byte] => {
            match msg.status() {
                Status::ProgramChange => patch::decode(*byte).map(SetPatch),
                _ => None,
            }
        }
        _ => None,
    }
}
