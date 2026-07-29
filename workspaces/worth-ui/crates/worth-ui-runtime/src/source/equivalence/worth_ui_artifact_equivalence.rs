use crate::source::{
    WorthUiArtifactDifference, WorthUiArtifactDigest, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactEquivalenceMetrics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactEquivalence {
    basis: WorthUiArtifactEquivalenceBasis,
    left_digest: WorthUiArtifactDigest,
    right_digest: WorthUiArtifactDigest,
    differences: Box<[WorthUiArtifactDifference]>,
    metrics: WorthUiArtifactEquivalenceMetrics,
}

impl WorthUiArtifactEquivalence {
    pub(crate) fn new(
        basis: WorthUiArtifactEquivalenceBasis,
        left_digest: WorthUiArtifactDigest,
        right_digest: WorthUiArtifactDigest,
        differences: Vec<WorthUiArtifactDifference>,
        metrics: WorthUiArtifactEquivalenceMetrics,
    ) -> Self {
        Self {
            basis,
            left_digest,
            right_digest,
            differences: differences.into_boxed_slice(),
            metrics,
        }
    }

    #[cfg(test)]
    pub(crate) fn basis(&self) -> WorthUiArtifactEquivalenceBasis {
        self.basis
    }

    #[cfg(test)]
    pub(crate) fn left_digest(&self) -> WorthUiArtifactDigest {
        self.left_digest
    }

    #[cfg(test)]
    pub(crate) fn right_digest(&self) -> WorthUiArtifactDigest {
        self.right_digest
    }

    pub(crate) fn is_equivalent(&self) -> bool {
        self.differences.is_empty() && self.left_digest == self.right_digest
    }

    pub(crate) fn first_difference(&self) -> Option<&WorthUiArtifactDifference> {
        self.differences.first()
    }

    pub(crate) fn differences(&self) -> &[WorthUiArtifactDifference] {
        &self.differences
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiArtifactEquivalenceMetrics {
        self.metrics
    }
}
