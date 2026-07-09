use crate::source::{WorthUiArtifactAssemblyDiagnostic, WorthUiArtifactAssemblyMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactAssemblyReport {
    diagnostics: Vec<WorthUiArtifactAssemblyDiagnostic>,
    metrics: WorthUiArtifactAssemblyMetrics,
}

impl WorthUiArtifactAssemblyReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiArtifactAssemblyDiagnostic>,
        metrics: WorthUiArtifactAssemblyMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| left.stable_cmp(right));
        Self {
            diagnostics,
            metrics,
        }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiArtifactAssemblyDiagnostic] {
        &self.diagnostics
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiArtifactAssemblyMetrics {
        self.metrics
    }
}
