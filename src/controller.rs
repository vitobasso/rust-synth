use std::sync::mpsc::{Receiver, SyncSender};
use synth::{
    instrument::Instrument,
    pitch::Pitch,
    oscillator::{Sine, Saw, Mix},
    filter::{BiquadFilter},
    arpeggiator::Arpeggiator,
};

type Sample = f64;
const PULSE_MILLIS: u64 = 100;

pub fn run_forever(sample_rate: f64, cmd_in: Receiver<Command>, signal_out: SyncSender<Sample>) {
    let mut state = State::new(sample_rate);

    loop {
        match cmd_in.try_recv() {
            Ok(command) => state.interpret(command),
            _ => (),
        }
        state.arpeggiator.as_mut()
            .and_then(|arp| arp.next())
            .map(|cmd| state.interpret(cmd));

        state.instrument.next_sample().map(|sample|
            signal_out.send(sample).expect("Failed to send a sample")
        );
    }
}

pub enum Command {
    Patch1, Patch2, Patch3, Patch4, Patch5, Patch6, Patch7, Patch8, Patch9, Patch0,
    NoteOn(Pitch), NoteOff(Pitch), ArpNoteOn(Pitch), ArpNoteOff(Pitch),
    Transpose(i8),
    ModParam1(f64), ModParam2(f64),
}

struct State {
    instrument: Instrument,
    arpeggiator: Option<Arpeggiator>,
}
impl State {
    fn new(sample_rate: f64) -> State {
        let instrument = Instrument::new(
            sample_rate,
            Box::new(Mix::supersaw(8, 3.0)),
            Box::new(BiquadFilter::lpf(sample_rate)),
        );
        State {
            instrument,
            arpeggiator: None,
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
                self.arpeggiator = Some(Arpeggiator::preset_3(PULSE_MILLIS));
            },
            Command::Patch8 => {
                self.arpeggiator = Some(Arpeggiator::preset_2(PULSE_MILLIS));
            },
            Command::Patch9 => {
                self.arpeggiator = Some(Arpeggiator::preset_1(PULSE_MILLIS));
            },
            Command::Patch0 => {
                self.arpeggiator = None;
            },
            Command::NoteOn(pitch) => {
                match self.arpeggiator.as_mut() {
                    Some(arp) => arp.start(pitch),
                    None => self.instrument.pitch = Some(pitch)
                }
            },
            Command::NoteOff(pitch) => {
                match self.arpeggiator.as_mut() {
                    Some(arp) =>
                        if arp.is_holding(pitch) {
                            arp.stop();
                            self.instrument.pitch = None
                        }
                    None =>
                        if self.instrument.pitch == Some(pitch) {
                            self.instrument.pitch = None
                        }
                }
            },
            Command::ArpNoteOn(pitch) => {
                self.instrument.pitch = Some(pitch);
            },
            Command::ArpNoteOff(pitch) => {
                if self.instrument.pitch == Some(pitch) {
                    self.instrument.pitch = None
                }
            },
            Command::Transpose(n) => {
                self.instrument.transpose = n;
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

}
