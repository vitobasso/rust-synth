use crate::core::{
    music_theory::{
        rhythm::{Note, Phrase, NoteDuration::*},
        diatonic_scale::{ScaleDegree::*, OctaveShift::*}
    },
    synth::{builder::*, lfo,
            instrument::{self, ModTarget::*},
            oscillator::{Specs::*, ModTarget::*},
            filter::ModTarget::*
    },
    control::tools::Patch,
};


pub fn instruments() -> Vec<instrument::Specs> {
    vec!(
        supersaw(),
        pulse(),
        sine(),
        saw_pad(),
    )
}

pub fn sequences() -> Vec<Phrase> {
    vec!(
        cyborg_chase(),
        topgear(),
        octaves(),
    )
}

pub fn patches() -> Vec<Patch> {
    vec!(
        Patch::Instrument(supersaw()),
        Patch::Instrument(pulse()),
        Patch::Instrument(sine()),
        Patch::Instrument(saw_pad()),
        Patch::Noop,
        Patch::Noop,
        Patch::Arpeggiator(Some(cyborg_chase())),
        Patch::Arpeggiator(Some(topgear())),
        Patch::Arpeggiator(Some(octaves())),
        Patch::Arpeggiator(None),
    )
}

pub fn sine() -> instrument::Specs {
    Builder::osc(Sine).mod_y(Volume).build()
}

pub fn pulse() -> instrument::Specs {
    Builder::osc(Pulse(0.5)).mod_y(Oscillator(PulseDuty)).build()
}

pub fn saw_pad() -> instrument::Specs {
    Builder::osc(Saw).adsr(0.25, 0., 1., 0.25).build()
}

pub fn supersaw() -> instrument::Specs {
    Builder::osc(Supersaw { nvoices: 8, detune_amount: 3., specs: Box::new(Saw)})
            .lfo(lfo::Specs::simple(0.1), Filter(Cutoff), 0.8).build()
}

fn octaves() -> Phrase {
    Phrase::new(&[
        Note::new(Eight, Down1, I1),
        Note::new(Eight, Same, I1),
        Note::new(Eight, Down1, I1),
        Note::new(Eight, Same, I1),
        Note::new(Eight, Down1, I1),
        Note::new(Eight, Same, I1),
        Note::new(Eight, Down1, I1),
        Note::new(Eight, Same, I1),
    ])
}


fn topgear() -> Phrase {
    Phrase::new(&[
        Note::new(Sixteenth, Down1, I1),
        Note::new(Sixteenth, Down1, I3),
        Note::new(Sixteenth, Down1, I5),
        Note::new(Sixteenth, Same, I1),
        Note::new(Sixteenth, Same, I3),
        Note::new(Sixteenth, Same, I5),
        Note::new(Sixteenth, Up1, I1),
        Note::new(Sixteenth, Up1, I3),
        Note::new(Sixteenth, Up1, I5),
        Note::new(Sixteenth, Up1, I3),
        Note::new(Sixteenth, Up1, I1),
        Note::new(Sixteenth, Same, I5),
        Note::new(Sixteenth, Same, I3),
        Note::new(Sixteenth, Same, I1),
        Note::new(Sixteenth, Down1, I5),
        Note::new(Sixteenth, Down1, I3),
    ])
}

fn cyborg_chase() -> Phrase {
    Phrase::new(&[
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Up1,  I4),
        Note::new(Sixteenth, Up1,  I3),
        Note::new(Sixteenth, Up1,  I1),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Up1,  I1),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Up1,  I2),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Same, I6),
        Note::new(Sixteenth, Up1,  I2),
        Note::new(Sixteenth, Up1,  I3),
    ])
}

