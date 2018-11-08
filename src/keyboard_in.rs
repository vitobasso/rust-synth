extern crate termion;

use Command;
use keyboard_in::termion::{
    event::Key, input::TermRead, raw::IntoRawMode, clear, cursor
};
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::Sender,
};

pub fn listen(cmd_out: Sender<Command>) {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Space plays, arrows change the sound.{}",
           clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();

    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", cursor::Goto(1, 1), clear::CurrentLine).unwrap();
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(' ') => {
                cmd_out.send(Command::NoteOn).expect("Failed to send Command::Space");
                println!("space");
            }
            Key::Up => {
                cmd_out.send(Command::Osc1).expect("Failed to send Command::Up");
                println!("up")
            },
            Key::Down => {
                cmd_out.send(Command::Osc2).expect("Failed to send Command::Down");
                println!("down")
            },
            _ => println!("?"),
        }
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", cursor::Show).unwrap();
}
