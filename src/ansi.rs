//! Ansi window

use std::io::{self, Write};
use termion::color;

/// Size
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// Position
pub struct Position {
    pub x: u16,
    pub y: u16,
}

struct Window {
    position: Position,
    size: Size,
    cursor_position: Position,
    background_color: String,

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn new(position: Position, size: Size) -> Self {
        Self {
            position,
            size,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn print(&mut self, s: &str) {
        let Position { mut x, mut y } = self.cursor_position;
        print!("{}", self.background_color);
        print!("{}", termion::cursor::Goto(x, y));

        let mut last_length = 0;

        for line in s.split("\n") {
            print!("{}", line);
            last_length = line.len();
            y += 1;
            print!("{}", termion::cursor::Goto(x, y));
        }
        x += last_length as u16;
        y -= 1;
        print!("{}", termion::cursor::Goto(x, y));
        self.cursor_position = Position { x, y };
    }

    #[allow(dead_code)]
    fn println(&mut self, s: &str) {
        self.print(s);
        self.cursor_position.x = self.position.x;
        self.cursor_position.y += 1;
        io::stdout().flush().unwrap();
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        let Position { x, y } = self.position;
        self.cursor_position = Position { x, y };
        print!("{}", self.background_color);

        let mut filler = String::new();
        for _ in 0..self.size.width {
            filler += " "
        }

        for i in 0..self.size.height {
            print!("{}", termion::cursor::Goto(x, y + i));
            print!("{}", filler);
        }

        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, Write};
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
        win.println("backstreets\nback");

        wait();
        print!("{}", color::Bg(color::Reset));
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        print!("{}", termion::cursor::Show);
    }

    fn wait() {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
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
