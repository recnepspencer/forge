use crate::source::{WorthUiStructuralLegalityDiagnostic, WorthUiStructuralLegalityMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiStructuralLegalityReport {
    diagnostics: Vec<WorthUiStructuralLegalityDiagnostic>,
    metrics: WorthUiStructuralLegalityMetrics,
}

impl WorthUiStructuralLegalityReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiStructuralLegalityDiagnostic>,
        metrics: WorthUiStructuralLegalityMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| left.stable_cmp(right));
        Self {
            diagnostics,
            metrics,
        }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiStructuralLegalityDiagnostic] {
        &self.diagnostics
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiStructuralLegalityMetrics {
        self.metrics
    }
}
