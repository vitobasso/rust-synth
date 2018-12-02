use std::sync::mpsc::{Receiver, SyncSender};
use synth::{
    Instrument,
    pitch::{Pitch, PitchClass},
    oscillator::{Sine, Saw, Mix},
    filter::{BiquadFilter},
    pulse::Pulse,
    arpeggiator::Arpeggiator,
};

type Sample = f64;

pub fn run_forever(sample_rate: f64, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate);
    let mut clock: f64 = 0.0;

    loop {
        match cmd_in.try_recv() {
            Ok(command) => state.interpret(command),
            _ => (),
        }
        match state.next_arpeggiator_command() {
            Some(command) => state.interpret(command),
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

//TODO move inside instrument
pub enum Command {
    Patch1, Patch2, Patch3, Patch4, Patch5, Patch6, Patch7, Patch8, Patch9, Patch0,
    NoteOn(Pitch), NoteOff(Pitch), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    Transpose(i8),
    ModParam1(f64), ModParam2(f64),
}

//TODO move inside instrument
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
        let arpeggiator = Arpeggiator::preset_1();
        State {
            note_on: false,
            transpose: 0_i8,
            instrument,
            pulse,
            arpeggiator,
            arpeggiator_on: false
        }
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Patch1 => {
                self.instrument.oscillator = Box::new(Sine)
            },
            Command::Patch2 => {
                self.instrument.oscillator = Box::new(Saw)
            },
            Command::Patch3 => {
                self.instrument.oscillator = Box::new(Mix::supersaw(8, 3.0))
            },
            Command::Patch7 => {
                self.arpeggiator = Arpeggiator::preset_3();
            },
            Command::Patch8 => {
                self.arpeggiator = Arpeggiator::preset_2();
            },
            Command::Patch9 => {
                self.arpeggiator = Arpeggiator::preset_1();
            },
            Command::Patch0 => {
                self.arpeggiator_on = !self.arpeggiator_on
            },
            Command::NoteOn(pitch) => {
                if self.arpeggiator_on {
                    self.arpeggiator.start(pitch);
                } else {
                    self.instrument.pitch = pitch + self.transpose;
                    self.note_on = true;
                }
            },
            Command::NoteOff(pitch) => {
                if self.arpeggiator_on && self.arpeggiator.is_holding(pitch){
                    self.arpeggiator.stop();
                    self.note_on = false;
                } else if self.instrument.pitch == pitch + self.transpose {
                    self.note_on = false;
                }
            },
            Command::ArpNoteOn(pitch) => {
                self.instrument.pitch = pitch + self.transpose;
                self.note_on = true;
            },
            Command::ArpNoteOff(pitch) => {
                if self.instrument.pitch == pitch + self.transpose {
                    self.note_on = false;
                }
            },
            Command::Transpose(n) => {
                self.transpose = self.transpose + n;
            },
            Command::ModParam1(value) => {
                self.instrument.set_mod_1(value);
            }
            Command::ModParam2(value) => {
                self.instrument.set_mod_2(value);
            },
            _ => (),
        }
    }

    fn next_arpeggiator_command(&mut self) -> Option<Command> {
        self.pulse.read()
            .and_then(|_| self.arpeggiator.next())
    }
}
