use conrod::{event, input};
use core::control::controller::{Command, Id};
use core::music_theory::pitch::{Pitch, PitchClass::*};

pub struct KeyMap { window_width: u32, window_height: u32 }
impl KeyMap {
    pub fn new(window_width: u32, window_height: u32) -> KeyMap {
        KeyMap { window_width, window_height }
    }
    pub fn command_for(&self, input: &event::Input) -> Vec<Command> {
        match input {
            event::Input::Press(input::Button::Keyboard(key)) =>
                note_on(key)
                    .or(patches(key))
                    .or(loop_rec(key))
                    .or(transpose(key))
                    .map_or(vec![], |v| vec![v]),
            event::Input::Release(input::Button::Keyboard(key)) =>
                note_off(key)
                    .map_or(vec![], |v| vec![v]),
            event::Input::Motion(input::Motion::MouseCursor {x, y}) => vec![self.mod_xy(*x, *y)],
            _ => vec![],
        }
    }

    fn mod_xy(&self, x: f64, y: f64) -> Command {
        let w = self.window_width as f64;
        let h = self.window_height as f64;
        let norm_y = ((y + h/2.)/h).max(0.).min(1.);
        let norm_x = ((x + w/2.)/w).max(0.).min(1.);
        Command::ModXY(norm_x, norm_y)
    }
}

fn note_on(key: &input::Key) -> Option<Command> {
    pitches(key).map(|(pitch, id)| Command::NoteOn(pitch,id))
}

fn note_off(key: &input::Key) -> Option<Command> {
    pitches(key).map(|(pitch, id)| Command::NoteOff(pitch,id))
}

fn pitches(key: &input::Key) -> Option<(Pitch, Id)> { //TODO shift => sharp pitches
    match key {
        //top row
        input::Key::Q =>         Some((Pitch::new(A, 4), 3)),
        input::Key::W =>         Some((Pitch::new(B, 4), 3)),
        input::Key::E =>         Some((Pitch::new(C, 5), 3)),
        input::Key::R =>         Some((Pitch::new(D, 5), 3)),
        input::Key::T =>         Some((Pitch::new(E, 5), 3)),
        input::Key::Y =>         Some((Pitch::new(F, 5), 3)),
        input::Key::U =>         Some((Pitch::new(G, 5), 3)),
        input::Key::I =>         Some((Pitch::new(A, 5), 3)),
        input::Key::O =>         Some((Pitch::new(B, 5), 3)),
        input::Key::P =>         Some((Pitch::new(C, 6), 3)),

        //middle row
        input::Key::A =>         Some((Pitch::new(A, 3), 2)),
        input::Key::S =>         Some((Pitch::new(B, 3), 2)),
        input::Key::D =>         Some((Pitch::new(C, 4), 2)),
        input::Key::F =>         Some((Pitch::new(D, 4), 2)),
        input::Key::G =>         Some((Pitch::new(E, 4), 2)),
        input::Key::H =>         Some((Pitch::new(F, 4), 2)),
        input::Key::J =>         Some((Pitch::new(G, 4), 2)),
        input::Key::K =>         Some((Pitch::new(A, 4), 2)),
        input::Key::L =>         Some((Pitch::new(B, 4), 2)),
        input::Key::Semicolon => Some((Pitch::new(C, 5), 2)),

        //bottom row
        input::Key::Z =>         Some((Pitch::new(A, 2), 1)),
        input::Key::X =>         Some((Pitch::new(B, 2), 1)),
        input::Key::C =>         Some((Pitch::new(C, 3), 1)),
        input::Key::V =>         Some((Pitch::new(D, 3), 1)),
        input::Key::B =>         Some((Pitch::new(E, 3), 1)),
        input::Key::N =>         Some((Pitch::new(F, 3), 1)),
        input::Key::M =>         Some((Pitch::new(G, 3), 1)),
        input::Key::Comma =>     Some((Pitch::new(A, 3), 1)),
        input::Key::Period =>    Some((Pitch::new(B, 3), 1)),
        input::Key::Slash =>     Some((Pitch::new(C, 4), 1)),

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

fn loop_rec(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::F1 =>  Some(Command::LoopPlaybackToggle(0)),
        input::Key::F2 =>  Some(Command::LoopPlaybackToggle(1)),
        input::Key::F3 =>  Some(Command::LoopPlaybackToggle(2)),
        input::Key::F4 =>  Some(Command::LoopPlaybackToggle(3)),
        input::Key::F5 =>  Some(Command::LoopPlaybackToggle(4)),
        input::Key::F6 =>  Some(Command::LoopRecordingToggle(0)),
        input::Key::F7 =>  Some(Command::LoopRecordingToggle(1)),
        input::Key::F8 =>  Some(Command::LoopRecordingToggle(2)),
        input::Key::F9 =>  Some(Command::LoopRecordingToggle(3)),
        input::Key::F10 => Some(Command::LoopRecordingToggle(4)),
        _ => None,
    }
}

fn transpose(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::Down =>         Some(Command::ShiftPitch(-12)),
        input::Key::Up =>           Some(Command::ShiftPitch(12)),
        input::Key::Left =>         Some(Command::ShiftKeyboard(-1)),
        input::Key::Right =>        Some(Command::ShiftKeyboard(1)),
        input::Key::Minus =>        Some(Command::ShiftPitch(-1)),
        input::Key::Equals =>       Some(Command::ShiftPitch(1)),
        input::Key::LeftBracket =>  Some(Command::TransposeKey(-1)),
        input::Key::RightBracket => Some(Command::TransposeKey(1)),
        _ => None,
    }
}
