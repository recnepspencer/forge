use crate::source::WorthUiArtifactEquivalenceBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDigest {
    basis: WorthUiArtifactEquivalenceBasis,
    raw: u64,
}

impl WorthUiArtifactDigest {
    pub(crate) fn new(basis: WorthUiArtifactEquivalenceBasis, raw: u64) -> Self {
        Self { basis, raw }
    }

    pub(crate) fn basis(&self) -> WorthUiArtifactEquivalenceBasis {
        self.basis
    }

    pub(crate) fn raw(self) -> u64 {
        self.raw
    }
}
