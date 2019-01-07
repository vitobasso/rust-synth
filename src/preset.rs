use core::{
    music_theory::{
        rhythm::{Sequence, Duration::*, note},
        diatonic_scale::{ScaleDegree::*, OctaveShift::*}
    },
    synth::{builder::*, lfo,
            instrument::{self, ModTarget::*},
            oscillator::{Specs::*, ModTarget::*},
            filter::{Specs::*, ModTarget::*}
    },
    control::manual_controller::Patch,
};


pub fn instruments() -> Vec<instrument::Specs> {
    vec!(
        supersaw(),
        pulse(),
        sine(),
        saw_pad(),
    )
}

pub fn sequences() -> Vec<Sequence> {
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

fn sine() -> instrument::Specs {
    Builder::osc(Sine).mod_y(Volume).build()
}

fn pulse() -> instrument::Specs {
    Builder::osc(Pulse(0.5)).mod_y(Oscillator(PulseDuty)).build()
}

fn saw_pad() -> instrument::Specs {
    Builder::osc(Saw).adsr(0.25, 0., 1., 0.25).build()
}

fn supersaw() -> instrument::Specs {
    Builder::osc(Supersaw { nvoices: 8, detune_amount: 3.})
            .lfo(lfo::Specs::simple(0.1), Filter(Cutoff), 0.8).build()
}

fn octaves() -> Sequence {
    Sequence::new(1, vec![
        note(Eight, (Down1, I1)),
        note(Eight, (Same, I1)),
        note(Eight, (Down1, I1)),
        note(Eight, (Same, I1)),
        note(Eight, (Down1, I1)),
        note(Eight, (Same, I1)),
        note(Eight, (Down1, I1)),
        note(Eight, (Same, I1)),
    ]).expect("Invalid sequence")
}


fn topgear() -> Sequence {
    Sequence::new(1, vec![
        note(Sixteenth, (Down1, I1)),
        note(Sixteenth, (Down1, I3)),
        note(Sixteenth, (Down1, I5)),
        note(Sixteenth, (Same, I1)),
        note(Sixteenth, (Same, I3)),
        note(Sixteenth, (Same, I5)),
        note(Sixteenth, (Up1, I1)),
        note(Sixteenth, (Up1, I3)),
        note(Sixteenth, (Up1, I5)),
        note(Sixteenth, (Up1, I3)),
        note(Sixteenth, (Up1, I1)),
        note(Sixteenth, (Same, I5)),
        note(Sixteenth, (Same, I3)),
        note(Sixteenth, (Same, I1)),
        note(Sixteenth, (Down1, I5)),
        note(Sixteenth, (Down1, I3)),
    ]).expect("Invalid sequence")
}

fn cyborg_chase() -> Sequence {
    Sequence::new(1, vec![
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Up1,  I4)),
        note(Sixteenth, (Up1,  I3)),
        note(Sixteenth, (Up1,  I1)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Up1,  I1)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Up1,  I2)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Same, I6)),
        note(Sixteenth, (Up1,  I2)),
        note(Sixteenth, (Up1,  I3)),
    ]).expect("Invalid sequence")
}

