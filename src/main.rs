#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[macro_use] extern crate num_derive;

use std::{thread, sync::mpsc::{channel, sync_channel}};
use core::{
    music_theory::{Hz, rhythm::{*, Duration::*}, diatonic_scale::{ScaleDegree::*, OctaveShift::*}},
    synth::{Sample, builder::*, instrument::ModTarget::*, oscillator::{Specs::*, ModTarget::*},
            lfo, filter::{Specs::*, ModTarget::*}},
    control::controller::{self, Patch, Command},
};

mod audio_out;
pub mod core;
mod gui;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let sample_rate = format.sample_rate.0 as Hz;
    let buffer_size = sample_rate as usize / 250;

    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<Sample>(buffer_size);

    thread::spawn(move || audio_out::run_forever(&device, &format, sig_in));
    thread::spawn(move || controller::run_forever(sample_rate, patches(), cmd_in, sig_out));
    gui::window::show(cmd_out);
}

fn patches() -> Vec<Patch> {

    let sine = Builder::osc(Sine).mod_y(Volume).build();
    let pulse = Builder::osc(Pulse(0.5)).mod_y(Oscillator(PulseDuty)).build();
    let saw_pad = Builder::osc(Saw).adsr(0.25, 0., 1., 0.25).build();
    let supersaw = Builder::osc(Supersaw { nvoices: 8, detune_amount: 3.})
        .lfo(lfo::Specs::simple(0.1), Filter(Cutoff), 0.8).build();

    let arp_1 = Sequence::new(1, vec![
        Note::note(Eight, (Down1, I1)),
        Note::note(Eight, (Same, I1)),
        Note::note(Eight, (Down1, I1)),
        Note::note(Eight, (Same, I1)),
        Note::note(Eight, (Down1, I1)),
        Note::note(Eight, (Same, I1)),
        Note::note(Eight, (Down1, I1)),
        Note::note(Eight, (Same, I1)),
    ]).expect("Invalid sequence");

    let arp_2 = Sequence::new(1, vec![
        Note::note(Sixteenth, (Down1, I1)),
        Note::note(Sixteenth, (Down1, I3)),
        Note::note(Sixteenth, (Down1, I5)),
        Note::note(Sixteenth, (Same, I1)),
        Note::note(Sixteenth, (Same, I3)),
        Note::note(Sixteenth, (Same, I5)),
        Note::note(Sixteenth, (Up1, I1)),
        Note::note(Sixteenth, (Up1, I3)),
        Note::note(Sixteenth, (Up1, I5)),
        Note::note(Sixteenth, (Up1, I3)),
        Note::note(Sixteenth, (Up1, I1)),
        Note::note(Sixteenth, (Same, I5)),
        Note::note(Sixteenth, (Same, I3)),
        Note::note(Sixteenth, (Same, I1)),
        Note::note(Sixteenth, (Down1, I5)),
        Note::note(Sixteenth, (Down1, I3)),
    ]).expect("Invalid sequence");

    vec![
        Patch::Instrument(supersaw),
        Patch::Instrument(pulse),
        Patch::Instrument(sine),
        Patch::Instrument(saw_pad),
        Patch::Noop,
        Patch::Noop,
        Patch::Noop,
        Patch::Arpeggiator(Some(arp_2)),
        Patch::Arpeggiator(Some(arp_1)),
        Patch::Arpeggiator(None),
    ]
}