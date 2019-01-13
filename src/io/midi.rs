use rimd;

use self::rimd::{SMF,SMFError, MidiMessage, Event, Status,
                 TrackEvent as RimdTrackEvent, Track as RimdTrack};
use std::path::Path;
use std::collections::HashMap;
use crate::core::control::{song::*, instrument_player::{Command, id}};
use crate::core::music_theory::pitch::*;

pub fn read_file(file_path: String) -> Option<Song> {
    println!("Reading midi file: {}", file_path);
    match SMF::from_file(&Path::new(&file_path[..])) {
        Ok(smf) =>
            Some(decode_sequence(&smf)),
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

fn decode_sequence(midi_file: &SMF) -> Song {
    let tracks = midi_file.tracks.iter().flat_map(decode_tracks).collect();
    Song { tracks }
}

fn decode_tracks(track: &RimdTrack) -> Vec<Track> {
    let mixed_events: Vec<(ScheduledCommand, Channel)> = track.events.iter()
        .filter_map(decode_track_event)
        .scan(0, |accumulated_time, ((cmd, time), channel)| {
            *accumulated_time += time;
            Some(((cmd, *accumulated_time), channel))
        })
        .collect();

    let events_by_channel: HashMap<Channel, Vec<ScheduledCommand>> = mixed_events.iter()
        .fold(HashMap::new(), |mut grouped, (cmd, channel)|{
            grouped.entry(*channel).or_insert(vec!()).push(*cmd);
            grouped
        });

    events_by_channel.into_iter()
        .map(|(channel, events)| Track::new(events, channel))
        .collect()
}

type Channel = u8;

fn decode_track_event(event: &RimdTrackEvent) -> Option<(ScheduledCommand, Channel)> {
    match event.event {
        Event::Midi(ref message) =>
            message.channel().and_then(|channel|
                decode_note_event(message)
                    .map(|cmd| ((cmd, event.vtime), channel))
            ),
        _ => None,
    }
}

fn decode_note_event(msg: &MidiMessage) -> Option<Command> {
    match msg.data.as_slice() {
        [_, pitch_byte, velocity_byte] => {
            let pitch = Pitch::from_index(*pitch_byte as usize);
            let velocity = *velocity_byte;
            let note_on = Command::NoteOn(pitch, id(pitch));
            let note_off = Command::NoteOff(id(pitch));
            match (msg.status(), velocity) {
                (Status::NoteOn, 0) => Some(note_off),
                (Status::NoteOn, _) => Some(note_on),
                (Status::NoteOff, _) => Some(note_off),
                _ => None,
            }
        },
        _ => None,
    }
}
