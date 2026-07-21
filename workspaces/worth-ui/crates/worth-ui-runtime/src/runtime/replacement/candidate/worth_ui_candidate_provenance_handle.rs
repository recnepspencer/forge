#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateProvenanceHandle {
    raw: u64,
}

impl WorthUiCandidateProvenanceHandle {
    pub(crate) fn new(raw: u64) -> Self {
        Self { raw }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }
}
