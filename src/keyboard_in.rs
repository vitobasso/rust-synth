extern crate termion;

use Command;
use keyboard_in::termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
};
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::Sender,
};

pub fn listen(cmd_out: Sender<Command>) {

    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Up => {
                cmd_out.send(Command::Up).expect("Failed to send Command::Up");
                println!("up")
            },
            Key::Down => {
                cmd_out.send(Command::Down).expect("Failed to send Command::Down");
                println!("down")
            },
            _ => println!("other"),
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
