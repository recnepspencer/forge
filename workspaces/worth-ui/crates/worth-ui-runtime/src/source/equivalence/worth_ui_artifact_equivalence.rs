use crate::source::{
    WorthUiArtifactDifference, WorthUiArtifactDigest, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactEquivalenceMetrics,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactEquivalence {
    basis: WorthUiArtifactEquivalenceBasis,
    left_digest: WorthUiArtifactDigest,
    right_digest: WorthUiArtifactDigest,
    first_difference: Option<WorthUiArtifactDifference>,
    metrics: WorthUiArtifactEquivalenceMetrics,
}

impl WorthUiArtifactEquivalence {
    pub(crate) fn new(
        basis: WorthUiArtifactEquivalenceBasis,
        left_digest: WorthUiArtifactDigest,
        right_digest: WorthUiArtifactDigest,
        first_difference: Option<WorthUiArtifactDifference>,
        metrics: WorthUiArtifactEquivalenceMetrics,
    ) -> Self {
        Self {
            basis,
            left_digest,
            right_digest,
            first_difference,
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
        self.first_difference.is_none() && self.left_digest == self.right_digest
    }

    pub(crate) fn first_difference(&self) -> Option<&WorthUiArtifactDifference> {
        self.first_difference.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiArtifactEquivalenceMetrics {
        self.metrics
    }
}
