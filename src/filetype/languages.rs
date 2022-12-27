use tree_sitter::Language;

/// Rust language
pub fn rust_language() -> Language {
    extern "C" {
        fn tree_sitter_rust() -> Language;
    }
    unsafe { tree_sitter_rust() }
}
