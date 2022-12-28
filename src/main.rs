#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::struct_excessive_bools)]

mod ansi;
mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
pub use editor::Color;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
use std::env;
pub use terminal::Terminal;

fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::default(args.get(1)).run();
}
