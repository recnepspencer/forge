use crate::source::{WorthUiResolutionDiagnostic, WorthUiResolutionMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiResolutionReport {
    diagnostics: Vec<WorthUiResolutionDiagnostic>,
    metrics: WorthUiResolutionMetrics,
}

impl WorthUiResolutionReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiResolutionDiagnostic>,
        metrics: WorthUiResolutionMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| left.stable_cmp(right));
        Self {
            diagnostics,
            metrics,
        }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiResolutionDiagnostic] {
        &self.diagnostics
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiResolutionMetrics {
        self.metrics
    }
}
