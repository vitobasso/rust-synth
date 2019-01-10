use conrod::{event, input};
use core::control::{controller::Command::{self, *}, instrument_player::{Command::*, Discriminator, id_discr},
                    loops::Command::*, transposer::Command::*};
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
                    .or(pulse_rec(key))
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
        Instrument(ModXY(norm_x, norm_y))
    }
}

fn note_on(key: &input::Key) -> Option<Command> {
    pitches(key).map(|(pitch, discr)|
        Instrument(NoteOn(pitch, id_discr(pitch, discr))))
}

fn note_off(key: &input::Key) -> Option<Command> {
    pitches(key).map(|(pitch, discr)|
        Instrument(NoteOff(id_discr(pitch, discr))))
}

fn pitches(key: &input::Key) -> Option<(Pitch, Discriminator)> { //TODO shift => sharp pitches
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
        input::Key::D1 => Some(SetPatch(0)),
        input::Key::D2 => Some(SetPatch(1)),
        input::Key::D3 => Some(SetPatch(2)),
        input::Key::D4 => Some(SetPatch(3)),
        input::Key::D5 => Some(SetPatch(4)),
        input::Key::D6 => Some(SetPatch(5)),
        input::Key::D7 => Some(SetPatch(6)),
        input::Key::D8 => Some(SetPatch(7)),
        input::Key::D9 => Some(SetPatch(8)),
        input::Key::D0 => Some(SetPatch(9)),
        _ => None,
    }
}

fn loop_rec(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::F1 =>  Some(Loop(TogglePlayback(0))),
        input::Key::F2 =>  Some(Loop(TogglePlayback(1))),
        input::Key::F3 =>  Some(Loop(TogglePlayback(2))),
        input::Key::F4 =>  Some(Loop(TogglePlayback(3))),
        input::Key::F5 =>  Some(Loop(TogglePlayback(4))),
        input::Key::F6 =>  Some(Loop(ToggleRecording(0))),
        input::Key::F7 =>  Some(Loop(ToggleRecording(1))),
        input::Key::F8 =>  Some(Loop(ToggleRecording(2))),
        input::Key::F9 =>  Some(Loop(ToggleRecording(3))),
        input::Key::F10 => Some(Loop(ToggleRecording(4))),
        _ => None,
    }
}

fn pulse_rec(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::Space =>  Some(PulseRecord),
        _ => None,
    }
}

fn transpose(key: &input::Key) -> Option<Command> {
    match key {
        input::Key::Down =>         Some(Transposer(ShiftPitch(-12))),
        input::Key::Up =>           Some(Transposer(ShiftPitch(12))),
        input::Key::Left =>         Some(Transposer(ShiftKeyboard(-1))),
        input::Key::Right =>        Some(Transposer(ShiftKeyboard(1))),
        input::Key::Minus =>        Some(Transposer(ShiftPitch(-1))),
        input::Key::Equals =>       Some(Transposer(ShiftPitch(1))),
        input::Key::LeftBracket =>  Some(Transposer(TransposeKey(-1))),
        input::Key::RightBracket => Some(Transposer(TransposeKey(1))),
        _ => None,
    }
}
