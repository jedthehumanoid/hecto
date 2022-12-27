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
    position: Position
    size: Size
    cursor_position: Position
}

impl Window {
fn println() {

}
fn print() {

}
fn clear() {

}
fn fg_color() {

}
fn bg_color() {

}
}
