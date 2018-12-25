use super::{
    Millis, pulse::Pulse, rhythm::*, diatonic_scale::Key,
    pitch::Pitch, controller::Command,
};

pub struct Arpeggiator {
    sequence: Sequence,
    index: usize,
    pulse: Pulse,
    key: Key,
    holding: Option<Pitch>,
    playing: Option<Pitch>,
}
impl Arpeggiator {
    pub fn new(pulse_period: Millis, key: Key, sequence: Sequence) -> Arpeggiator {
        Arpeggiator {
            sequence,
            index: 0,
            pulse: Pulse::with_period_millis(pulse_period),
            key,
            holding: None,
            playing: None,
        }
    }
    pub fn start(&mut self, pitch: Pitch) {
        self.holding = Some(pitch);
    }

    pub fn stop(&mut self) {
        self.holding = None;
        self.playing = None;
    }

    pub fn is_holding(&self, pitch: Pitch) -> bool {
        self.holding.map(|p| p == pitch).unwrap_or(false)
    }

    pub fn next(&mut self) -> Option<Command> {
        self.pulse.read()
            .and_then(|_| self.next_event())
            .and_then(|e| self.update_and_command(e))
    }

    fn next_event(&mut self) -> Option<Event> {
        let events = &self.sequence.events;
        self.index = (self.index + 1) % events.len();
        events.iter().nth(self.index).cloned()
    }

    fn update_and_command(&mut self, event: Event) -> Option<Command> {
        match (event, self.holding, self.playing) {
            (Event::Note(relative_pitch), Some(holding), _) => {
                self.key.pitch_at(holding, relative_pitch).map(|pitch| {
                    self.playing = Some(pitch);
                    Command::ArpNoteOn(pitch)
                })
            },
            (Event::Rest, _, Some(playing)) => {
                self.playing = None;
                Some(Command::ArpNoteOff(playing))
            }
            _ => None,
        }
    }

}
