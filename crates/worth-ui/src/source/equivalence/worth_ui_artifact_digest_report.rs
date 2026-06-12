use crate::source::{WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceMetrics};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDigestReport {
    basis: WorthUiArtifactEquivalenceBasis,
    metrics: WorthUiArtifactEquivalenceMetrics,
}

impl WorthUiArtifactDigestReport {
    pub(crate) fn new(
        basis: WorthUiArtifactEquivalenceBasis,
        metrics: WorthUiArtifactEquivalenceMetrics,
    ) -> Self {
        Self { basis, metrics }
    }

    pub(crate) fn basis(&self) -> WorthUiArtifactEquivalenceBasis {
        self.basis
    }

    pub(crate) fn metrics(&self) -> WorthUiArtifactEquivalenceMetrics {
        self.metrics
    }
}
