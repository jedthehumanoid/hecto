//! Highlighting types and colors for the types.
use termion::color;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
            Type::PrimaryKeywords => color::Rgb(181, 137, 0),
            Type::SecondaryKeywords => color::Rgb(42, 161, 152),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}

pub fn comment(hl: &mut Vec<Type>, index: &mut usize, chars: &[char]) -> bool {
    if chars[*index] == '/' && *index < chars.len() {
        if let Some(next_char) = chars.get(index.saturating_add(1)) {
            if *next_char == '/' {
                for _ in *index..chars.len() {
                    hl.push(Type::Comment);
                    *index += 1;
                }

                return true;
            }
        };
    }
    false
}

#[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
pub fn multiline_comment(hl: &mut Vec<Type>, index: &mut usize, chars: &[char]) -> bool {
    if chars[*index] == '/' && *index < chars.len() {
        if let Some(next_char) = chars.get(index.saturating_add(1)) {
            if *next_char == '*' {
                let closing_index = if let Some(closing_index) = String::from_iter(chars).find("*/")
                {
                    *index + closing_index + 4
                } else {
                    chars.len()
                };
                for _ in *index..closing_index {
                    hl.push(Type::MultilineComment);
                    *index += 1;
                }
                return true;
            }
        };
    }
    false
}

pub fn char(hl: &mut Vec<Type>, index: &mut usize, chars: &[char]) -> bool {
    if chars[*index] == '\'' {
        if let Some(next_char) = chars.get(index.saturating_add(1)) {
            let closing_index = if *next_char == '\\' {
                index.saturating_add(3)
            } else {
                index.saturating_add(2)
            };
            if let Some(closing_char) = chars.get(closing_index) {
                if *closing_char == '\'' {
                    for _ in 0..=closing_index.saturating_sub(*index) {
                        hl.push(Type::Character);
                        *index += 1;
                    }
                    return true;
                }
            }
        }
    }
    false
}

pub fn number(hl: &mut Vec<Type>, index: &mut usize, chars: &[char]) -> bool {
    if chars[*index].is_ascii_digit() {
        if *index > 0 {
            #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }
        loop {
            hl.push(Type::Number);
            *index += 1;
            if let Some(next_char) = chars.get(*index) {
                if *next_char != '.' && !next_char.is_ascii_digit() {
                    break;
                }
            } else {
                break;
            }
        }
        return true;
    }
    false
}

pub fn string(hl: &mut Vec<Type>, index: &mut usize, chars: &[char]) -> bool {
    if chars[*index] != '"' {
        return false;
    }
    loop {
        hl.push(Type::String);
        *index += 1;
        if let Some(next_char) = chars.get(*index) {
            if *next_char == '"' {
                break;
            }
        } else {
            break;
        }
    }
    hl.push(Type::String);
    *index += 1;
    true
}

fn keyword(
    hl: &mut Vec<Type>,
    index: &mut usize,
    substring: &str,
    chars: &[char],
    hl_type: Type,
) -> bool {
    if substring.is_empty() {
        return false;
    }
    for (substring_index, c) in substring.chars().enumerate() {
        if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
            if *next_char != c {
                return false;
            }
        } else {
            return false;
        }
    }
    for _ in 0..substring.len() {
        hl.push(hl_type);
        *index += 1;
    }
    true
}
pub fn keywords(
    hl: &mut Vec<Type>,
    index: &mut usize,
    chars: &[char],
    keywords: &[String],
    hl_type: Type,
) -> bool {
    if *index > 0 {
        #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
        let prev_char = chars[*index - 1];
        if !is_separator(prev_char) {
            return false;
        }
    }
    for word in keywords {
        if *index < chars.len().saturating_sub(word.len()) {
            #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
            let next_char = chars[*index + word.len()];
            if !is_separator(next_char) {
                continue;
            }
        }

        if keyword(hl, index, word, chars, hl_type) {
            return true;
        }
    }
    false
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}
