use std::sync::mpsc::{Receiver, Sender, SyncSender};
use synth::{
    Instrument,
    pitch::{Pitch, PitchClass},
    oscillator::{Sine, Saw, Mix},
    filter::{BiquadFilter},
    pulse::Pulse,
    arpeggiator::{self, Arpeggiator},
    rhythm::Sequence,
};

type Sample = f64;

pub fn run_forever(sample_rate: f64,
                   cmd_in: Receiver<Command>,
                   signal_out: SyncSender<Sample>,
                   update_out: Sender<StateUpdate>) {

    let mut state = State::new(sample_rate);
    update_out.send(StateUpdate::Oscillator(OscillatorType::Supersaw));
    update_out.send(StateUpdate::FilterType(FilterType::LPF));
    update_out.send(StateUpdate::FilterParams(1., 0.));
    update_out.send(StateUpdate::ArpeggiatorToggle(false));
    update_out.send(StateUpdate::Key(PitchClass::C));
    let mut clock: f64 = 0.;

    loop {
        match cmd_in.try_recv() {
            Ok(command) => {
                state.interpret(command)
                    .map(|update| {
                        update_out.send(update)
                    });
            },
            _ => (),
        }
        match state.next_arpeggiator_command() {
            Some(command) => {
                state.interpret(command);
            },
            _ => (),
        }

        clock = clock + 1.0;
        if state.note_on {
            let normalized_clock: f64 = clock/sample_rate;
            let sample: Sample = state.instrument.next_sample(normalized_clock);
            signal_out.send(sample).expect("Failed to send a sample");
        }
    }
}

pub enum Command {
    Patch1, Patch2, Patch3, Patch4, Patch5, Patch6, Patch7, Patch8, Patch9, Patch0,
    NoteOn(Pitch), NoteOff(Pitch), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    Transpose(i8),
    ModParam1(f64), ModParam2(f64),
}
#[derive(Debug)]
pub enum OscillatorType { Sine, Saw, Supersaw }
#[derive(Debug)]
pub enum FilterType { LPF, HPF, BPF, Notch }
#[derive(Debug)]
pub enum StateUpdate {
    Oscillator(OscillatorType),
    FilterType(FilterType),
    FilterParams(f64, f64),
    ArpeggiatorToggle(bool),
    ArpeggiatorSeq(Sequence),
    Key(PitchClass),
}

struct State {
    note_on: bool,
    transpose: i8,
    instrument: Instrument,
    pulse: Pulse,
    arpeggiator: Arpeggiator,
    arpeggiator_on: bool,
}
impl State {
    fn new(sample_rate: f64) -> State {
        let instrument = Instrument::new(
            sample_rate,
            Box::new(Mix::supersaw(8, 3.0)),
            Box::new(BiquadFilter::lpf()),
        );
        let pulse = Pulse::with_period_millis(100);
        let arpeggiator = arpeggiator::Builder::preset_1().build();
        State {
            note_on: false,
            transpose: 0_i8,
            instrument,
            pulse,
            arpeggiator,
            arpeggiator_on: false
        }
    }

    fn interpret(&mut self, command: Command) -> Option<StateUpdate> {
        match command {
            Command::Patch1 => {
                self.instrument.oscillator = Box::new(Sine);
                Some(StateUpdate::Oscillator(OscillatorType::Sine))
            },
            Command::Patch2 => {
                self.instrument.oscillator = Box::new(Saw);
                Some(StateUpdate::Oscillator(OscillatorType::Saw))
            },
            Command::Patch3 => {
                self.instrument.oscillator = Box::new(Mix::supersaw(8, 3.0));
                Some(StateUpdate::Oscillator(OscillatorType::Supersaw))
            },
            Command::Patch7 => {
                let builder = arpeggiator::Builder::preset_3();
                self.arpeggiator = builder.build();
                Some(StateUpdate::ArpeggiatorSeq(builder.sequence))
            },
            Command::Patch8 => {
                let builder = arpeggiator::Builder::preset_2();
                self.arpeggiator = builder.build();
                Some(StateUpdate::ArpeggiatorSeq(builder.sequence))
            },
            Command::Patch9 => {
                let builder = arpeggiator::Builder::preset_1();
                self.arpeggiator = builder.build();
                Some(StateUpdate::ArpeggiatorSeq(builder.sequence))
            },
            Command::Patch0 => {
                self.arpeggiator_on = !self.arpeggiator_on;
                Some(StateUpdate::ArpeggiatorToggle(self.arpeggiator_on))
            },
            Command::NoteOn(pitch) => {
                if self.arpeggiator_on {
                    self.arpeggiator.start(pitch);
                } else {
                    self.instrument.pitch = pitch + self.transpose;
                    self.note_on = true;
                }
                None
            },
            Command::NoteOff(pitch) => {
                if self.arpeggiator_on && self.arpeggiator.is_holding(pitch){
                    self.arpeggiator.stop();
                    self.note_on = false;
                } else if self.instrument.pitch == pitch + self.transpose {
                    self.note_on = false;
                }
                None
            },
            Command::ArpNoteOn(pitch) => {
                self.instrument.pitch = pitch + self.transpose;
                self.note_on = true;
                None
            },
            Command::ArpNoteOff(pitch) => {
                if self.instrument.pitch == pitch + self.transpose {
                    self.note_on = false;
                }
                None
            },
            Command::Transpose(n) => {
                self.transpose = self.transpose + n;
                Some(StateUpdate::Key(PitchClass::C + self.transpose))
            },
            Command::ModParam1(value) => {
                self.instrument.set_mod_1(value);
                Some(StateUpdate::FilterParams(self.instrument.mod_1, self.instrument.mod_2))
            },
            Command::ModParam2(value) => {
                self.instrument.set_mod_2(value);
                Some(StateUpdate::FilterParams(self.instrument.mod_1, self.instrument.mod_2))
            },
            _ => None,
        }
    }

    fn next_arpeggiator_command(&mut self) -> Option<Command> {
        self.pulse.read()
            .and_then(|_| self.arpeggiator.next())
    }
}
