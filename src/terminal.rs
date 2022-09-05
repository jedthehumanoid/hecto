#![warn(clippy::missing_docs_in_private_items)]

//! Abstracts terminal
use crate::Position;
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

/// Terminal
pub struct Terminal {
    /// Size of terminal
    size: Size,

    /// Unused handle to keep stdout in raw mode
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// Default constructor
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    /// Get size
    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Clear entire screen
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// Set cursor position
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    // Should not be public?
    /// Flush terminal
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    ///
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    /// Hide cursor
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    /// Show cursor
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    /// Clear current line
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// Set background color
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    /// Reset background color
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    /// Set foreground color
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    /// Reset foreground color
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
