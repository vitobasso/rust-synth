use conrod::{event, input};
use controller::Command;
use pitches::{Pitch, PitchClass};

pub fn command_for(input: &event::Input) -> Option<Command> {
    match input {
        event::Input::Press(input::Button::Keyboard(key)) =>
            pitches(key).map(Command::NoteOn)
                .or(patches(key))
                .or(transpose(key)),
        event::Input::Release(input::Button::Keyboard(key)) =>
            pitches(key).map(Command::NoteOff),
        _ => None,
    }
}

fn pitches(key: &input::Key) -> Option<Pitch> { //TODO shift => sharp pitches
    match key {
        //top row
        input::Key::Q => Some(Pitch { class: PitchClass::A, octave: 4 }),
        input::Key::W => Some(Pitch { class: PitchClass::B, octave: 4 }),
        input::Key::E => Some(Pitch { class: PitchClass::C, octave: 5 }),
        input::Key::R => Some(Pitch { class: PitchClass::D, octave: 5 }),
        input::Key::T => Some(Pitch { class: PitchClass::E, octave: 5 }),
        input::Key::Y => Some(Pitch { class: PitchClass::F, octave: 5 }),
        input::Key::U => Some(Pitch { class: PitchClass::G, octave: 5 }),
        input::Key::I => Some(Pitch { class: PitchClass::A, octave: 5 }),
        input::Key::O => Some(Pitch { class: PitchClass::B, octave: 5 }),
        input::Key::P => Some(Pitch { class: PitchClass::C, octave: 6 }),

        //middle row
        input::Key::A => Some(Pitch { class: PitchClass::A, octave: 3 }),
        input::Key::S => Some(Pitch { class: PitchClass::B, octave: 3 }),
        input::Key::D => Some(Pitch { class: PitchClass::C, octave: 4 }),
        input::Key::F => Some(Pitch { class: PitchClass::D, octave: 4 }),
        input::Key::G => Some(Pitch { class: PitchClass::E, octave: 4 }),
        input::Key::H => Some(Pitch { class: PitchClass::F, octave: 4 }),
        input::Key::J => Some(Pitch { class: PitchClass::G, octave: 4 }),
        input::Key::K => Some(Pitch { class: PitchClass::A, octave: 4 }),
        input::Key::L => Some(Pitch { class: PitchClass::B, octave: 4 }),
        input::Key::Semicolon => Some(Pitch { class: PitchClass::C, octave: 5 }),

        //bottom row
        input::Key::Z => Some(Pitch { class: PitchClass::A, octave: 2 }),
        input::Key::X => Some(Pitch { class: PitchClass::B, octave: 2 }),
        input::Key::C => Some(Pitch { class: PitchClass::C, octave: 3 }),
        input::Key::V => Some(Pitch { class: PitchClass::D, octave: 3 }),
        input::Key::B => Some(Pitch { class: PitchClass::E, octave: 3 }),
        input::Key::N => Some(Pitch { class: PitchClass::F, octave: 3 }),
        input::Key::M => Some(Pitch { class: PitchClass::G, octave: 3 }),
        input::Key::Comma => Some(Pitch { class: PitchClass::A, octave: 3 }),
        input::Key::Period => Some(Pitch { class: PitchClass::B, octave: 3 }),
        input::Key::Slash => Some(Pitch { class: PitchClass::C, octave: 4 }),

        _ => None,
    }
}

fn patches(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::D1 => Some(Command::Osc1),
        input::Key::D2 => Some(Command::Osc2),
        input::Key::D3 => Some(Command::Osc3),
        input::Key::D4 => Some(Command::Osc4),
        input::Key::D5 => Some(Command::Osc5),
        input::Key::D6 => Some(Command::Osc6),
        input::Key::D7 => Some(Command::Osc7),
        input::Key::D8 => Some(Command::Osc8),
        input::Key::D9 => Some(Command::Osc9),
        input::Key::D0 => Some(Command::Osc0),
        _ => None,
    }
}

fn transpose(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::LeftBracket => Some(Command::Transpose(-1)),
        input::Key::RightBracket => Some(Command::Transpose(1)),
        //TODO shift brackets => octave up/down
        _ => None,
    }
}
