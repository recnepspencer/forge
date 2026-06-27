#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiArtifactSubtreeDigest {
    raw: u64,
}

impl WorthUiArtifactSubtreeDigest {
    pub(crate) fn new(raw: u64) -> Self {
        Self { raw }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}
