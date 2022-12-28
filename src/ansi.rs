//! Abstracts terminal
#![warn(clippy::missing_docs_in_private_items)]

use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// Size
pub struct Size {
    /// Width
    pub width: u16,

    /// Height
    pub height: u16,
}

/// Position
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Window {
    position: Position,
    size: Size,
    cursor_position: Position,
    background_color: String,
    foreground_color: String,
}

impl Default for Window {
    fn default() -> Self {
        let size = termion::terminal_size().unwrap();
        Self {
            position: Position { x: 1, y: 1 },
            size: Size {
                width: size.0,
                height: size.1,
            },
            cursor_position: Position { x: 1, y: 1 },
            background_color: format!("{}", color::Bg(color::Reset)),
            foreground_color: format!("{}", color::Fg(color::Reset)),
        }
    }
}

impl Window {
    fn new(position: Position, size: Size) -> Self {
        Self {
            position: position,
            size: size,
            ..Default::default()
        }
    }

    fn print(&mut self, s: &str) {
        print!("{}", self.background_color);
        print!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
        );

        let mut last_length = 0;

        for x in s.split("\n") {
            print!("{}", x);
            last_length = x.len();
            self.cursor_position.y += 1;
            print!(
                "{}",
                termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
            );
        }
        self.cursor_position.x += last_length as u16;
        self.cursor_position.y -= 1;
        print!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
        );
    }

    fn println(&mut self, s: &str) {
        print!("{}", self.background_color);
        print!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
        );

        for x in s.split("\n") {
            print!("{}", x);
            self.cursor_position.y += 1;
            print!(
                "{}",
                termion::cursor::Goto(self.cursor_position.x, self.cursor_position.y)
            );
        }
    }
    fn clear(&mut self) {
        self.cursor_position = Position {
            x: self.position.x,
            y: self.position.y,
        };

        print!("{}", self.background_color);

        let mut filler = String::new();

        for _ in 0..self.size.width {
            filler += " "
        }

        for i in 0..self.size.height {
            print!(
                "{}",
                termion::cursor::Goto(self.position.x, self.position.y + i)
            );
            print!("{}", filler);
        }
        self.cursor_position = Position {
            x: self.position.x,
            y: self.position.y,
        };

        io::stdout().flush();
    }
    fn fg_color() {}
    fn bg_color() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, stdout, Write};
    use termion::event::{Event, Key, MouseEvent};
    use termion::input::{MouseTerminal, TermRead};
    use termion::raw::IntoRawMode;
    #[test]
    fn clear() {
        print!("{}", termion::cursor::Hide);
        let mut win = Window::new(
            Position { x: 10, y: 10 },
            Size {
                width: 10,
                height: 10,
            },
        );
        win.background_color = format!("{}", color::Bg(color::Rgb(247, 134, 170)));
        win.clear();

        win.print("hello\n..");
        win.print("hello?");
        win.print("ya");

        let mut win = Window::new(
            Position { x: 30, y: 10 },
            Size {
                width: 20,
                height: 10,
            },
        );
        win.background_color = format!("{}", color::Bg(color::Rgb(40, 150, 90)));
        win.clear();

        win.println("hello world");
        win.println("backstreets back");
        io::stdout().flush();
        wait();
        print!("{}", color::Bg(color::Reset));
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        print!("{}", termion::cursor::Show);
    }
    fn wait() {
        let stdin = stdin();
        let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => break,
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}
