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
}

impl Default for Window {
    fn default() -> Self {
        let size = termion::terminal_size().unwrap();
        Self {
            position: Position{x: 0,y: 0},
            size: Size{width: size.0, height: size.1},
            cursor_position: Position{x: 0, y:0},
        }
    }
}

impl Window {
fn println() {

}
fn print() {

}
fn clear(&self) {
    
}
fn fg_color() {

}
fn bg_color() {

}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let win = Window::default();
        win.clear();
    }
}