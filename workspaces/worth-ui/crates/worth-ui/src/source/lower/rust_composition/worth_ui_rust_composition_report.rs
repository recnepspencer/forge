use crate::source::{WorthUiArtifactInput, WorthUiRustCompositionMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRustCompositionReport {
    artifact_input: WorthUiArtifactInput,
    metrics: WorthUiRustCompositionMetrics,
}

impl WorthUiRustCompositionReport {
    pub(super) fn new(
        artifact_input: WorthUiArtifactInput,
        metrics: WorthUiRustCompositionMetrics,
    ) -> Self {
        Self {
            artifact_input,
            metrics,
        }
    }

    pub(crate) fn artifact_input(&self) -> &WorthUiArtifactInput {
        &self.artifact_input
    }

    pub(crate) fn into_artifact_input(self) -> WorthUiArtifactInput {
        self.artifact_input
    }

    pub(crate) fn metrics(&self) -> WorthUiRustCompositionMetrics {
        self.metrics
    }
}
