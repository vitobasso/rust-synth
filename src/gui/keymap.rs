use conrod::{event, input};
use synth::controller::Command;
use synth::pitch::{Pitch, PitchClass::*};

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
        input::Key::Q =>         Some(Pitch::new(A, 4)),
        input::Key::W =>         Some(Pitch::new(B, 4)),
        input::Key::E =>         Some(Pitch::new(C, 5)),
        input::Key::R =>         Some(Pitch::new(D, 5)),
        input::Key::T =>         Some(Pitch::new(E, 5)),
        input::Key::Y =>         Some(Pitch::new(F, 5)),
        input::Key::U =>         Some(Pitch::new(G, 5)),
        input::Key::I =>         Some(Pitch::new(A, 5)),
        input::Key::O =>         Some(Pitch::new(B, 5)),
        input::Key::P =>         Some(Pitch::new(C, 6)),

        //middle row
        input::Key::A =>         Some(Pitch::new(A, 3)),
        input::Key::S =>         Some(Pitch::new(B, 3)),
        input::Key::D =>         Some(Pitch::new(C, 4)),
        input::Key::F =>         Some(Pitch::new(D, 4)),
        input::Key::G =>         Some(Pitch::new(E, 4)),
        input::Key::H =>         Some(Pitch::new(F, 4)),
        input::Key::J =>         Some(Pitch::new(G, 4)),
        input::Key::K =>         Some(Pitch::new(A, 4)),
        input::Key::L =>         Some(Pitch::new(B, 4)),
        input::Key::Semicolon => Some(Pitch::new(C, 5)),

        //bottom row
        input::Key::Z =>         Some(Pitch::new(A, 2)),
        input::Key::X =>         Some(Pitch::new(B, 2)),
        input::Key::C =>         Some(Pitch::new(C, 3)),
        input::Key::V =>         Some(Pitch::new(D, 3)),
        input::Key::B =>         Some(Pitch::new(E, 3)),
        input::Key::N =>         Some(Pitch::new(F, 3)),
        input::Key::M =>         Some(Pitch::new(G, 3)),
        input::Key::Comma =>     Some(Pitch::new(A, 3)),
        input::Key::Period =>    Some(Pitch::new(B, 3)),
        input::Key::Slash =>     Some(Pitch::new(C, 4)),

        _ => None,
    }
}

fn patches(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::D1 => Some(Command::SetPatch(0)),
        input::Key::D2 => Some(Command::SetPatch(1)),
        input::Key::D3 => Some(Command::SetPatch(2)),
        input::Key::D4 => Some(Command::SetPatch(3)),
        input::Key::D5 => Some(Command::SetPatch(4)),
        input::Key::D6 => Some(Command::SetPatch(5)),
        input::Key::D7 => Some(Command::SetPatch(6)),
        input::Key::D8 => Some(Command::SetPatch(7)),
        input::Key::D9 => Some(Command::SetPatch(8)),
        input::Key::D0 => Some(Command::SetPatch(9)),
        _ => None,
    }
}

fn transpose(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::Down => Some(Command::ShiftPitch(-12)),
        input::Key::Up => Some(Command::ShiftPitch(12)),
        input::Key::Left => Some(Command::ShiftKeyboard(-1)),
        input::Key::Right => Some(Command::ShiftKeyboard(1)),
        input::Key::Minus => Some(Command::ShiftPitch(-1)),
        input::Key::Equals => Some(Command::ShiftPitch(1)),
        input::Key::LeftBracket => Some(Command::TransposeKey(-1)),
        input::Key::RightBracket => Some(Command::TransposeKey(1)),
        _ => None,
    }
}
