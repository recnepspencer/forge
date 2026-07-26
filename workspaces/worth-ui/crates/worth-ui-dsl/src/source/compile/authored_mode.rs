/// Diagnostic evidence describing how one semantic package was authored.
///
/// Runtime admission may inspect this posture, but it may not choose a
/// different semantic preparation path from it.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAuthoredMode {
    File,
    Rust,
}

impl WorthUiAuthoredMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Rust => "rust",
        }
    }
}
