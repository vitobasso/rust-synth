use num_traits::FromPrimitive;
use crate::core::synth::instrument::Specs;
use crate::preset;
use self::PatchCategory::*;

pub fn decode(program_byte: u8) -> Option<Specs> {
    decode_category(program_byte)
        .map(patch_to_specs)
}

fn patch_to_specs(patch: Patch) -> Specs {
    match patch.category {
        SynthLead | Piano | Guitar | Bass | SynthEffects | Ensemble => preset::supersaw(),
        ChromaticPercussion => preset::pulse(),
        Organ | Reed | Pipe => preset::sine(),
        Strings | SynthPad => preset::saw_pad(),
        _ => preset::sine(),
    }
}

fn decode_category(byte: u8) -> Option<Patch> {
    let group_index = byte / 8;
    FromPrimitive::from_u8(group_index)
        .map(|category| Patch { category, specific: byte % 8 })
}

#[derive(Clone, Copy, Debug)]
struct Patch {
    category: PatchCategory,
    specific: u8,
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
enum PatchCategory {
    Piano,
    ChromaticPercussion,
    Organ,
    Guitar,
    Bass,
    Strings,
    Ensemble,
    Reed,
    Pipe,
    SynthLead,
    SynthPad,
    SynthEffects,
    Ethnic,
    Percussive,
    SoundEffects,
}
