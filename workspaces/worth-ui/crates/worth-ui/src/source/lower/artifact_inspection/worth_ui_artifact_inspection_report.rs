use crate::source::{WorthUiArtifactInspectionDiagnostic, WorthUiArtifactInspectionMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactInspectionReport {
    diagnostics: Vec<WorthUiArtifactInspectionDiagnostic>,
    metrics: WorthUiArtifactInspectionMetrics,
}

impl WorthUiArtifactInspectionReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiArtifactInspectionDiagnostic>,
        metrics: WorthUiArtifactInspectionMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| left.stable_cmp(right));
        Self {
            diagnostics,
            metrics,
        }
    }

    pub(crate) fn diagnostics(&self) -> &[WorthUiArtifactInspectionDiagnostic] {
        &self.diagnostics
    }

    pub(crate) fn metrics(&self) -> WorthUiArtifactInspectionMetrics {
        self.metrics
    }
}
