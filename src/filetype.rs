//! Wraps filetype and highlighting options of filetype
#![warn(clippy::missing_docs_in_private_items)]

mod languages;

/// Friendly name of filetype and highlighting options
pub struct FileType {
    /// Friendly name of file type
    name: String,

    /// Highlighting options of file type
    hl_opts: HighlightingOptions,

    ts_language: Option<tree_sitter::Language>,
}

/// Wether to highlight different syntax elements
#[derive(Default)]
pub struct HighlightingOptions {
    /// Supports numbers highlighting
    numbers: bool,

    /// Supports strings highlightling
    strings: bool,

    /// Supports single characters highlighting
    characters: bool,

    /// Supports single line comments highlighting
    comments: bool,

    /// Multiline comments highlighting support
    multiline_comments: bool,

    /// Primary keywords highlighting support
    primary_keywords: Vec<String>,

    /// Secondary keywords highlighting support
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
            ts_language: None,
        }
    }
}

impl FileType {
    /// Return name of filetype
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn parse(&self, contents: &str) -> Option<tree_sitter::Tree> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(self.ts_language?).ok()?;

        parser
            .parse(contents, None)
    }

    /// Return highlighting options of filetype
    #[must_use]
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }

    /// From filename, return name of filetype and highlighting options
    #[must_use]
    pub fn from(file_name: &str) -> Self {
        let file_name = std::path::Path::new(file_name);
        if file_name
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            return Self {
                name: String::from("Rust"),
                ts_language: Some(languages::rust_language()),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    /// Whether filetype supports numbers highlighting
    #[must_use]
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    /// Whether filetype supports strings highlighting
    #[must_use]
    pub fn strings(&self) -> bool {
        self.strings
    }

    /// Whether filetype supports characters highlighting
    #[must_use]
    pub fn characters(&self) -> bool {
        self.characters
    }

    /// Whether filetype supports comments highlighting
    #[must_use]
    pub fn comments(&self) -> bool {
        self.comments
    }

    /// Filetype primary keywords
    #[must_use]
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    /// Filtype secondary keywords
    #[must_use]
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    /// Whether filetype supports multiline comments highlighting
    #[must_use]
    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
