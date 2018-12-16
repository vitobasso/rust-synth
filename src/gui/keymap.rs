use conrod::{event, input};
use synth::controller::Command;
use synth::pitch::{Pitch, PitchClass};

pub fn command_for(input: &event::Input) -> Vec<Command> {
    match input {
        event::Input::Press(input::Button::Keyboard(key)) =>
            pitches(key).map(Command::NoteOn)
                .or(patches(key))
                .or(transpose(key))
                .map_or(vec![], |v| vec![v]),
        event::Input::Release(input::Button::Keyboard(key)) =>
            pitches(key).map(Command::NoteOff)
                .map_or(vec![], |v| vec![v]),
        event::Input::Motion(input::Motion::MouseCursor {x, y}) => {
            let norm_y = ((y + 100.0)/200.0).max(0.).min(1.); //TODO get from screen size
            let norm_x = ((x + 200.0)/400.0).max(0.).min(1.); //TODO get from screen size
            vec!(Command::ModXY(norm_x, norm_y))
        }
        _ => vec![],
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
        input::Key::D1 => Some(Command::SetPatch(1)),
        input::Key::D2 => Some(Command::SetPatch(2)),
        input::Key::D3 => Some(Command::SetPatch(3)),
        input::Key::D4 => Some(Command::SetPatch(4)),
        input::Key::D5 => Some(Command::SetPatch(5)),
        input::Key::D6 => Some(Command::SetPatch(6)),
        input::Key::D7 => Some(Command::SetPatch(7)),
        input::Key::D8 => Some(Command::SetPatch(8)),
        input::Key::D9 => Some(Command::SetPatch(9)),
        input::Key::D0 => Some(Command::SetPatch(0)),
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
