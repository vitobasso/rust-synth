use super::rimd::{MetaEvent, MetaCommand};
use crate::core::control::song::Tempo;
use std::mem;

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

pub fn decode(msg: &MetaEvent) -> Option<Meta> {
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
