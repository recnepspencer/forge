use crate::source::{WorthUiArtifactDependencyMetrics, WorthUiIncrementalInvalidationBasis};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDependencyReport {
    basis: WorthUiIncrementalInvalidationBasis,
    metrics: WorthUiArtifactDependencyMetrics,
}

impl WorthUiArtifactDependencyReport {
    pub(crate) fn new(
        basis: WorthUiIncrementalInvalidationBasis,
        metrics: WorthUiArtifactDependencyMetrics,
    ) -> Self {
        Self { basis, metrics }
    }

    pub(crate) fn basis(&self) -> &WorthUiIncrementalInvalidationBasis {
        &self.basis
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiArtifactDependencyMetrics {
        self.metrics
    }
}
