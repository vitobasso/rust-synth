#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[macro_use] extern crate num_derive;

use std::{thread, sync::mpsc::{channel, sync_channel}};
use core::{
    music_theory::{Hz, rhythm::{*, Duration::*}, diatonic_scale::{ScaleDegree::*, OctaveShift::*}},
    synth::{Sample, instrument::{self, Modulation::*}, oscillator::{Specs::*, Modulation::*},
            filter::{Specs::*, Modulation::*}, envelope::Adsr},
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

    let adsr_smooth = Adsr::new(0.05, 0.2, 0.9, 0.5);
    let adsr_plucked = Adsr::new(0., 0.05, 0.8, 0.2);
    let osc_supersaw = Supersaw { n_voices: 8, detune_amount: 3. };
    let sine = instrument::Specs { max_voices: 8, oscillator: Sine, filter: LPF, adsr: adsr_smooth, volume: 1.2, x_modulation: Noop, y_modulation: Noop };
    let saw = instrument::Specs { max_voices: 8, oscillator: Saw, filter: LPF, adsr: adsr_smooth, volume: 1., x_modulation: Noop, y_modulation: Noop };
    let pulse = instrument::Specs { max_voices: 8, oscillator: Pulse(0.5), filter: LPF, adsr: adsr_smooth, volume: 1., x_modulation: Filter(Cutoff), y_modulation: Oscillator(PulseDuty) };
    let supersaw = instrument::Specs { max_voices: 8, oscillator: osc_supersaw, filter: LPF, adsr: adsr_plucked, volume: 0.4, x_modulation: Filter(Cutoff), y_modulation: Filter(QFactor) };

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
        Patch::Instrument(sine),
        Patch::Instrument(pulse),
        Patch::Instrument(saw),
        Patch::Instrument(supersaw),
        Patch::Noop,
        Patch::Noop,
        Patch::Noop,
        Patch::Arpeggiator(Some(arp_2)),
        Patch::Arpeggiator(Some(arp_1)),
        Patch::Arpeggiator(None),
    ]
}