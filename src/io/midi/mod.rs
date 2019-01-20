use rimd;

use self::rimd::{SMF, SMFError, MidiMessage, Status,
                 Event as RimdEvent, TrackEvent as RimdTrackEvent, Track as RimdTrack};
use std::path::Path;
use std::collections::HashMap;
use crate::core::control::{
    song::*,
    instrument_player::{id, Command, Command::*},
};
use crate::core::music_theory::pitch::*;
use self::meta_events::Meta;

mod patch;
mod meta_events;

pub fn read_file(file_path: &str) -> Vec<Song> {
    println!("MIDI: Reading file: {}", file_path);
    match SMF::from_file(&Path::new(&file_path[..])) {
        Ok(smf) =>
            decode_midi_file(&smf)
        ,
        Err(e) => {
            match e {
                SMFError::InvalidSMFFile(s) => println!("Invalid Midi file: {}", s),
                SMFError::Error(e) => println!("IO Error: {}", e),
                SMFError::MidiError(e) => println!("Midi Error: {}", e),
                SMFError::MetaError(_) => println!("Meta Error"),
            };
            vec!()
        }
    }
}

fn decode_midi_file(midi_file: &SMF) -> Vec<Song> {
    assert!(midi_file.division > 0, "MIDI: Unsupported format. Header has negative division.");
    let ticks_per_beat: u16 = midi_file.division as u16;
    midi_file.tracks.iter().map(|track| decode_track(track, ticks_per_beat)).collect()
}

fn decode_track(track: &RimdTrack, ticks_per_beat: u16) -> Song {
    let mixed_events: Vec<Event> = decode_track_events(track, ticks_per_beat);
    let (commands_by_channel, meta_events) = group_track_events(mixed_events);

    let mut song = group_meta_events(meta_events);
    song.voices = commands_by_channel.into_iter()
        .map(|(channel, events)| Voice::new(events, channel))
        .collect();
    song
}

fn group_meta_events(events: Vec<ScheduledMeta>) -> Song {
    let mut title: String = String::from("Unnamed");
    let mut tempo: Tempo = DEFAULT_TEMPO;
    let mut end = 0;
    let mut key = PitchClass::C;
    for (meta, time) in events.into_iter() {
        match meta {
            Meta::TrackName(name) => title = name,
            Meta::KeySignature { sharps, minor } => key = convert_key_signature(sharps, minor),
            Meta::TempoSetting(t) => tempo = t,
            Meta::EndOfTrack => end = time,
            _ => (),
        }
    }
    Song { title, key, tempo, end, voices: vec!() }
}

fn convert_key_signature(sharps: i8, minor: bool) -> PitchClass {
    let offset = if minor { PitchClass::A } else { PitchClass::C };
    offset.circle_of_fifths(sharps)
}

fn group_track_events(events: Vec<Event>) -> (HashMap<Channel, Vec<ScheduledCommand>>, Vec<ScheduledMeta>) {
    let mut commands_by_channel: HashMap<Channel, Vec<ScheduledCommand>> = HashMap::default();
    let mut meta_events: Vec<ScheduledMeta> = Vec::default();
    for event in events.into_iter() {
        match event {
            Event::Midi(cmd, channel) =>
                commands_by_channel.entry(channel).or_insert_with(|| vec!()).push(cmd),
            Event::Meta(meta) =>
                meta_events.push(meta),
        }
    }
    (commands_by_channel, meta_events)
}

fn decode_track_events(track: &RimdTrack, ticks_per_beat: u16) -> Vec<Event> {
    let events = track.events.iter()
        .filter_map(decode_track_event)
        .collect();
    accumulate_time(events, ticks_per_beat)
}

fn accumulate_time(events: Vec<Event>, ticks_per_beat: u16) -> Vec<Event> {
    let time_stretch = DEFAULT_TICKS_PER_BEAT as f64 / ticks_per_beat as f64;
    events.into_iter()
        .scan(0, |accumulated_time, event| match event {
            Event::Midi((cmd, time), channel) => {
                *accumulated_time += (time as f64 * time_stretch) as u64;
                Some(Event::Midi((cmd, *accumulated_time), channel))
            },
            Event::Meta((cmd, time)) => {
                *accumulated_time += (time as f64 * time_stretch) as u64;
                Some(Event::Meta((cmd, *accumulated_time)))
            }
        }).collect()
}

type Channel = u8;
type ScheduledMeta = (Meta, Tick);
enum Event {
    Midi(ScheduledCommand, Channel),
    Meta(ScheduledMeta)
}

fn decode_track_event(event: &RimdTrackEvent) -> Option<Event> {
    match event.event {
        RimdEvent::Midi(ref message) =>
            message.channel().and_then(|channel|
                decode_note_event(message)
                    .map(|cmd| ((cmd, event.vtime), channel))
            ).map(|(cmd, channel)|Event::Midi(cmd, channel)),
        RimdEvent::Meta(ref meta) => {
            meta_events::decode(meta).map(|meta| Event::Meta((meta, event.vtime)))
        },

    }
}

fn decode_note_event(msg: &MidiMessage) -> Option<Command> {
    match msg.data.as_slice() {
        [_, pitch_byte, velocity_byte] => {
            let pitch = Pitch::from_index(*pitch_byte as usize);
            let velocity = *velocity_byte;
            let note_on = NoteOn(pitch, id(pitch));
            let note_off = NoteOff(id(pitch));
            match (msg.status(), velocity) {
                (Status::NoteOn, 0) => Some(note_off),
                (Status::NoteOn, _) => Some(note_on),
                (Status::NoteOff, _) => Some(note_off),
                _ => {
                    eprintln!("MIDI: Ignored note event: {:?}, data={:?}", msg.status(), msg.data);
                    None
                }
            }
        }
        [_, byte] => {
            match msg.status() {
                Status::ProgramChange => patch::decode(*byte).map(SetPatch),
                _ => {
                    eprintln!("MIDI: Ignored note event: {:?}, data={:?}", msg.status(), msg.data);
                    None
                }
            }
        }
        _ => {
            eprintln!("MIDI: Ignored note event: {:?}, data={:?}", msg.status(), msg.data);
            None
        }
    }
}
