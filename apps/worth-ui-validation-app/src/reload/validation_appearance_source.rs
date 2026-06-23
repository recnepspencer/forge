use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAppearanceSource {
    source_path: PathBuf,
    source_text: String,
    source_digest: u64,
}

impl ValidationAppearanceSource {
    pub fn new(source_text: impl Into<String>) -> Self {
        Self::from_observed_file(
            "apps/worth-ui-validation-app/theme/header.appearance",
            source_text,
        )
    }

    pub fn from_observed_file(
        source_path: impl Into<PathBuf>,
        source_text: impl Into<String>,
    ) -> Self {
        let source_text = source_text.into();
        Self {
            source_path: source_path.into(),
            source_digest: fold_bytes(0xcbf2_9ce4_8422_2325, source_text.as_bytes()),
            source_text,
        }
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
