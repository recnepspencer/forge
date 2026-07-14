use crate::source::{WorthUiIdentitySeedingDiagnostic, WorthUiIdentitySeedingMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiIdentitySeedingReport {
    diagnostics: Vec<WorthUiIdentitySeedingDiagnostic>,
    metrics: WorthUiIdentitySeedingMetrics,
}

impl WorthUiIdentitySeedingReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiIdentitySeedingDiagnostic>,
        metrics: WorthUiIdentitySeedingMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| {
            left.code()
                .cmp(&right.code())
                .then_with(|| left.module_id().cmp(right.module_id()))
                .then_with(|| left.authored_identity().cmp(right.authored_identity()))
                .then_with(|| left.semantic_locus().cmp(right.semantic_locus()))
                .then_with(|| left.conflicting_locus().cmp(right.conflicting_locus()))
        });
        Self {
            diagnostics,
            metrics,
        }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiIdentitySeedingDiagnostic] {
        &self.diagnostics
    }

    #[cfg(test)]
    pub(crate) fn metrics(&self) -> WorthUiIdentitySeedingMetrics {
        self.metrics
    }
}
