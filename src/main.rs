extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut note = String::new();

    write!(stdout,
            "{}{}Just start typing. Use Tab to indent and Shift+Tab to unindent. Type ESC to exit.{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Char(c) => {
                match c {
                    '\n' => {
                        note.push_str("\r\n");
                    },
                    _ => note.push(c)
                }
            },
            Key::Backspace => {
                if note.len() > 0 {
                    if note.pop().expect("") == '\n' {
                        note = note.trim_end().to_string();
                    }
                }
            },
            _ => {}
        }

        write!(stdout, "{}{}{}{}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(1, 1),
                note,
                termion::cursor::Show)
                .unwrap();

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1))
        .unwrap();
}